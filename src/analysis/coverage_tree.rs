use std::collections::HashSet;
use std::collections::{HashMap, hash_map::Entry};

use std::fmt::Debug;

use std::hash::Hash;
use std::str::FromStr;

use chrono::{NaiveTime, NaiveDateTime, Datelike, NaiveDate, Duration, Days};

use crate::rotations::manifest::Manifest;
use crate::rotations::rotation_error::RotationManifestParseError;
use crate::rotations::timespan::{parse_time_span, midnight};
use crate::{processed_source::ProcessedSource, globals::{main_headers, SITES, MODALITIES, tpc_headers, business_days}, constraints::ConstraintSet, dates::business_days_per_year, categorization::{buildSalemRVUMap, buildSalemBVUMap}};

use super::source_error::SourceError;

pub struct CoverageCoordinates
{
    site:String,
    subspecialty:String,
    context:String,
    modality:String,
    weekday:chrono::Weekday
}

impl Default for CoverageCoordinates
{
    fn default() -> Self {
        Self { site: Default::default(), subspecialty: Default::default(), context: Default::default(), modality: Default::default(), weekday: chrono::Weekday::Sun }
    }
}

#[derive(Default,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub struct CoverageUnit
{
    start:NaiveTime, //Make first so it's sorted on start time first!
    end:NaiveTime, //Make second so it's sorted on end time next!
    rotation:String
}

#[derive(Default,Debug)]
pub struct WorkUnit
{
    datetime:NaiveDateTime,
    rvu:f64,
    bvu:f64
}

#[derive(Default,Debug)]
pub struct CoverageAndWorkDay
{
    coverages:Vec<CoverageUnit>,
    work:Vec<WorkUnit>
}

#[derive(Default)]
pub struct CoverageErrors
{
    gaps:Vec<(NaiveTime,NaiveTime)>,
    overlaps:Vec<(String,String)>
}

impl CoverageErrors
{
    pub fn concat(&mut self,other:&mut Self)->()
    {
        self.gaps.append(&mut other.gaps);
        self.overlaps.append(&mut other.overlaps);
    }
}

impl CoverageAndWorkDay
{
    fn audit_coverage(&mut self)->CoverageErrors
    {
        let mut retval=CoverageErrors::default();

        if self.coverages.is_empty()
        {
            retval.gaps.push((midnight(),midnight()));
            return retval;
        }
        
        self.coverages.sort(); //Sorting puts them in order with respect to start time and then end time!

        let mut prior=CoverageUnit
        {
            start:midnight(),
            end:midnight(),
            rotation:"".to_string()
        };

        for cu in &self.coverages
        {
            //Check overlap
            if cu.start<prior.end
            {
                retval.overlaps.push((prior.rotation.to_string(),cu.rotation.to_string()));
            }
            //Check gap
            if(cu.start>prior.end)
            {
                retval.gaps.push((prior.end,cu.start));
            }
        }

        //Check through midnight
        let last = self.coverages.last().expect("Shouldn't be empty!").end;
        if last != midnight()
        {
            retval.gaps.push((last,midnight()));
        }
        
        retval
    }
}

pub trait WorkCoverageMap
{
    fn add_work(&mut self,coords:&CoverageCoordinates,work:WorkUnit);
    fn add_coverage(&mut self,coords:&CoverageCoordinates,coverage:CoverageUnit);
}
pub trait  CoordinateMap<'a,T,U> 
    where T:'a + Debug + Eq + PartialEq + Hash,
    U:Default+Debug
{
    fn get_map(&mut self)->&mut HashMap<T,U>;
    fn get_coordinate(coords:&CoverageCoordinates)->T;
    fn get_branch(&'a mut self, coords:&'a CoverageCoordinates)->&mut U
    {
        let key=Self::get_coordinate(&coords);
        let retval = match (*self.get_map()).entry(key)
        {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(U::default())
        };
        retval
    }
}

#[derive(Default, Debug)]
pub struct WeekdayMap {
    map:HashMap<chrono::Weekday,CoverageAndWorkDay>
}
impl WeekdayMap
{
    fn default()->WeekdayMap{
        let mut map:HashMap<chrono::Weekday,CoverageAndWorkDay>=HashMap::new();
        map.insert(chrono::Weekday::Mon, CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Tue,  CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Wed,  CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Thu,  CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Fri,  CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Sat,  CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Sun,  CoverageAndWorkDay::default());
        WeekdayMap { 
            map: map
        }
    }
}
impl WorkCoverageMap for WeekdayMap
{
    fn add_work(&mut self,coords:&CoverageCoordinates,work:WorkUnit){
        self.get_branch(coords).work.push(work);
    }
    fn add_coverage(&mut self,coords:&CoverageCoordinates,coverage:CoverageUnit){
        self.get_branch(coords).coverages.push(coverage);
    }
}
impl <'a> CoordinateMap<'a,chrono::Weekday,CoverageAndWorkDay> for WeekdayMap
{
    fn get_map(&mut self)->&mut HashMap<chrono::Weekday,CoverageAndWorkDay> {
        &mut self.map
    }
    fn get_coordinate(coords:&CoverageCoordinates)->chrono::Weekday {
        coords.weekday.clone()
    }
}


#[derive(Default, Debug)]
pub struct ModalityMap {
    map:HashMap<String, WeekdayMap>
}
impl <'a> CoordinateMap<'a,String,WeekdayMap> for ModalityMap
{
    fn get_map(&mut self)->&mut HashMap<String,WeekdayMap> {
        &mut self.map
    }
    fn get_coordinate(coords:&CoverageCoordinates) -> String {
        coords.modality.clone()
    }
}
impl WorkCoverageMap for ModalityMap{
    fn add_work(&mut self,coords:&CoverageCoordinates,work:WorkUnit) {
        self.get_branch(coords).add_work(coords, work)
    }

    fn add_coverage(&mut self,coords:&CoverageCoordinates,coverage:CoverageUnit) {
        self.get_branch(coords).add_coverage(coords, coverage)
    }
}

#[derive(Default, Debug)]
pub struct ContextMap {
    map:HashMap<String, ModalityMap>
}
impl <'a> CoordinateMap<'a,String,ModalityMap> for ContextMap
{
    fn get_map(&mut self)->&mut HashMap<String,ModalityMap> {
        &mut self.map
    }
    fn get_coordinate(coords:&CoverageCoordinates) -> String {
        coords.context.clone()
    }
}
impl WorkCoverageMap for ContextMap{
    fn add_work(&mut self,coords:&CoverageCoordinates,work:WorkUnit) {
        self.get_branch(coords).add_work(coords, work)
    }

    fn add_coverage(&mut self,coords:&CoverageCoordinates,coverage:CoverageUnit) {
        self.get_branch(coords).add_coverage(coords, coverage)
    }
}


#[derive(Default,Debug)]
pub struct SubspecialtyMap {
     map:HashMap<String, ContextMap>
}
impl <'a> CoordinateMap<'a,String,ContextMap> for SubspecialtyMap
{
    fn get_map(&mut self)->&mut HashMap<String,ContextMap> {
        &mut self.map
    }
    fn get_coordinate(coords:&CoverageCoordinates)->String {
        coords.subspecialty.clone()
    }
}
impl WorkCoverageMap for SubspecialtyMap{
    fn add_work(&mut self,coords:&CoverageCoordinates,work:WorkUnit) {
        self.get_branch(coords).add_work(coords, work)
    }

    fn add_coverage(&mut self,coords:&CoverageCoordinates,coverage:CoverageUnit) {
        self.get_branch(coords).add_coverage(coords, coverage)
    }
}


#[derive(Default)]
pub struct CoverageMap {
    map:HashMap<String, SubspecialtyMap>
}
impl CoverageMap
{
   pub fn add_work_from_source<'a>(&mut self, source:ProcessedSource, date_constraints:&ConstraintSet<'a,NaiveDateTime>)->Result<(),Box<dyn std::error::Error>>
   {
    let mut retval=CoverageMap::default();

    let mut modality_map:HashMap<String,String>=HashMap::new();

    let exam_rvu_map=buildSalemRVUMap(&source.main_data_table)?;
    let exam_bvu_map: HashMap<String, f64>=buildSalemBVUMap(&source.bvu_data_table)?;
    
    let mut salem_weekday_count :HashMap<chrono::Weekday,u64>=HashMap::new();
    //Determine how many days worth for each weekday
    {
        let mut dateset:HashSet<NaiveDate>=HashSet::new();
        for row_i in source.main_data_table.rowIndices()
        {
            let datetimestring= source.main_data_table.getVal(&main_headers::pertinent_headers::scheduled_datetime.getLabel(), &row_i)?;
            
            let datetime=match NaiveDateTime::parse_from_str(&datetimestring, "%m/%d/%y %H:%M"){
                Ok(x)=>x,
                Err(x)=>{return Err(Box::new(x));}
            };

            if date_constraints.include(&datetime)
            {
                dateset.insert(NaiveDate::from(datetime));
            }
        }

        for date in dateset
        {
            match salem_weekday_count.entry(date.weekday())
            {
                Entry::Occupied(x) => {
                    let mutable = x.into_mut();
                    *mutable+=1;
                },
                Entry::Vacant(x) => {
                    x.insert(1);
                },
            };
        }
    }
    //Process Salem Data
    for row_i in source.main_data_table.rowIndices()
    {
        let datetimestring= source.main_data_table.getVal(&main_headers::pertinent_headers::scheduled_datetime.getLabel(), &row_i)?;
        
        let datetime=match NaiveDateTime::parse_from_str(&datetimestring, "%m/%d/%y %H:%M"){
            Ok(x)=>x,
            Err(x)=>{return Err(Box::new(x));}
        };

        if date_constraints.include(&datetime)
        {
            let denominator = *salem_weekday_count.get(&NaiveDate::from(datetime).weekday()).expect("All weekdays should be populated") as f64;

            let location=source.main_data_table.getVal(&main_headers::pertinent_headers::location.getLabel(), &row_i)?;
            let exam_code=source.main_data_table.getVal(&main_headers::pertinent_headers::procedure_code.getLabel(), &row_i)?;

            //Build coords and populate maps with this row.
            let coords:CoverageCoordinates =
            {
                //Get subspecialty from exam code
                let subspecialty=match source.exam_to_subspecialty_map.get(&exam_code)
                {
                    Some(x)=>x.to_string(),
                    None=>{
                        return SourceError::generate_boxed(format!("Invalid exam_code {} in exam_to_subspeciality_map",exam_code));
                    }
                };


                //Try site. If not valid, go by location.
                let mut selected_site:Option<String>=None;
                let listed_site=source.main_data_table.getVal(&main_headers::pertinent_headers::accession.getLabel(), &row_i)?;
                for site in SITES
                {
                    if (listed_site[0..site.len()]).to_ascii_uppercase()==site.to_string().to_ascii_uppercase()
                    {
                        selected_site=Some(site.to_string());
                        break;
                    }
                }
                if selected_site.is_none()
                {
                    selected_site=crate::globals::getLocationSiteMapping(&location);  
                }
                let site=match selected_site
                {
                    Some(x)=>x,
                    None=>{
                        return SourceError::generate_boxed(format!("Could not determine site for row {}",row_i));
                    }
                };

                //Try context. If not valid, go by site map.
                let context= match source.location_to_context_map.get(&location)
                {
                    Some(x)=>x.to_string(),
                    None=>{    
                        match crate::globals::getLocationSiteMapping(&location)
                        {
                            Some(x)=>x,
                            None=>{
                                return SourceError::generate_boxed(format!("Could not determine context for location {}",location));
                            }
                        }
                    }
                };          

                //Get modality, but check for aliases
                let listed_modality = source.main_data_table.getVal(&main_headers::pertinent_headers::modality.getLabel(), &row_i)?;
                let mut selected_modality:Option<String>=None;
                for modality in MODALITIES
                {
                    if modality.to_string()==listed_modality
                    {
                        selected_modality=Some(modality.to_string());
                        break;
                    }
                }
                match selected_modality
                {
                    None=>{
                        selected_modality=crate::globals::getModalityAlias(&listed_modality);
                    },
                    _=>{}
                }
                match selected_modality
                {
                    None=>{
                        selected_modality=crate::globals::getModalityFromProcedureDesc(source.main_data_table.getVal(&main_headers::pertinent_headers::exam.getLabel(), &row_i)?)
                    },
                    _=>{}
                }
                let modality=match selected_modality
                {
                    Some(x)=>x,
                    None=>{
                        return SourceError::generate_boxed(format!("Could not determine modality for row {}",row_i));
                    }
                };
                if !modality_map.contains_key(&exam_code)
                {
                    modality_map.insert(exam_code.to_owned(), modality.to_string());
                }

                CoverageCoordinates{
                    site:site,
                    subspecialty:subspecialty,
                    context:context,
                    modality:modality.to_string(),
                    weekday:datetime.weekday(),
                }
            };

            let work:WorkUnit =
            {
                let rvu = match exam_rvu_map.get(&exam_code)
                {
                    Some(x)=>x,
                    None=>{return SourceError::generate_boxed(format!("Invalid exam_code {} in exam_to_subspeciality_map",exam_code));}
                };

                let bvu=match  exam_bvu_map.get(&exam_code)
                {
                    Some(x)=>x,
                    None=>{return SourceError::generate_boxed(format!("Invalid exam_code {} in exam_to_subspeciality_map",exam_code));}
                };

                WorkUnit {
                    datetime:datetime,
                    rvu:*rvu/denominator, //divide by denominator to account for aggregation of many days of data
                    bvu:*bvu/denominator, //divide by denominator to account for aggregation of many days of data
                }
            };

            self.add_work(&coords,work);
        }
    }
    //Add TPC, which doesn't go by number of dates
    let weights=crate::time::getNormalDistWeights();
    for row_i in source.tpc_data_table.rowIndices()
    {
        let exam_code = source.tpc_data_table.getVal(&tpc_headers::pertinent_headers::exam_code.getLabel(),&row_i)?;
        let number_str = source.tpc_data_table.getVal(&tpc_headers::pertinent_headers::number_in_2022.getLabel(),&row_i)?;
        
        let number=match number_str.parse::<f64>(){
            Ok(val)=>val,
            Err(e)=>{return SourceError::generate_boxed(format!("{:?}",e));}
        };

        let number_per_business_day=number/business_days_per_year;

        let rvus_per_exam=match exam_rvu_map.get(&exam_code){
            Some(val)=>val.to_owned(),
            None=>{return SourceError::generate_boxed(format!("Bad exam code {}",exam_code));}
        };
        let bvus_per_exam=match exam_bvu_map.get(&exam_code){
            Some(val)=>val.to_owned(),
            None=>{return SourceError::generate_boxed(format!("Bad exam code {}",exam_code));}
        };
        
        let rvus_per_business_day =number_per_business_day*rvus_per_exam;
        let bvus_per_business_day =number_per_business_day*bvus_per_exam;

        let subspecialty = match source.exam_to_subspecialty_map.get(&exam_code){
            None=>{return SourceError::generate_boxed(format!("Bad exam code {}",exam_code));}
            Some(val)=>val.to_owned()
        };

        let modality = match modality_map.get(&exam_code)
        {
            None=>{return SourceError::generate_boxed(format!("Bad exam code {}",exam_code));}
            Some(val)=>val.to_owned()
        };

        for weekday in business_days
        {
            let coords=CoverageCoordinates{
                site: crate::globals::TPC.to_string(),
                context: crate::globals::Outpatient.to_string(),
                modality:modality.to_string(),
                subspecialty:subspecialty.to_string(),              
                weekday: **weekday
            };

            let mut date = NaiveDate::default();
            date=date+Duration::days(**weekday as i64-date.weekday()as i64);

            if date.weekday()!=**weekday
            {
                return SourceError::generate_boxed(format!("Weekday math is wrong."));
            }

            for key in weights.keys() {
                let work=WorkUnit {
                    datetime:NaiveDateTime::new(date, *key),
                    rvu:rvus_per_business_day*(*weights.get(key).expect("Expected")) as f64,
                    bvu:bvus_per_business_day*(*weights.get(key).expect("Expected")) as f64
                };
                self.add_work(&coords,work);
            }
        }        
    }

    Ok(())

   }

   pub fn add_coverage_from_manifest(&mut self, manifest:Manifest)->Result<(),Box<dyn std::error::Error>>
   {
    let mut coords:CoverageCoordinates=CoverageCoordinates::default();
    for rotation_description in &manifest.rotation_manifest
    {
        for responsibility in &rotation_description.responsibilities
        {
            for site in responsibility.sites.to_vec()
            {
                coords.site=site.to_string();
                for subspecialty in responsibility.subspecialties.to_vec()
                {
                    coords.subspecialty=subspecialty.to_string();
                    for context in responsibility.contexts.to_vec()
                    {
                        coords.context=context.to_string();
                        for modality in responsibility.modalities.to_vec()
                        {
                            coords.modality=modality.to_string();
                            for weekday_string in responsibility.days.to_vec()
                            {
                                let weekday = match chrono::Weekday::from_str(&weekday_string){
                                    Ok(x) => x,
                                    Err(_) => return RotationManifestParseError::generate_boxed(0,"".to_string()),
                                };
                                for time_period in responsibility.time_periods.to_vec()
                                {
                                    let timespan = parse_time_span(time_period.as_str()).expect("Erroneous timespan in manifest.");
                                    let periods = timespan.instantiate_periods(weekday);
                                    for (day,start,end) in periods
                                    {
                                        coords.weekday=day;
                                        let coverage:CoverageUnit=CoverageUnit{
                                            start:start,
                                            end:end,
                                            rotation:rotation_description.rotation.to_string()
                                        };
                                        self.add_coverage(&coords, coverage)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
   }

   pub fn audit(&mut self)->Vec<String>
   {
    let mut retval:Vec<String> = Vec::new();
    for (site, subspecialtymap) in self.map.iter_mut()
    {
        for(subspecialty, contextmap) in subspecialtymap.map.iter_mut()
        {
            for(context, modalitymap) in contextmap.map.iter_mut()
            {
                for(modality, weekdaymap) in modalitymap.map.iter_mut()
                {
                    for(weekday, coverage_and_workday) in weekdaymap.map.iter_mut()
                    {
                        let errs=coverage_and_workday.audit_coverage();

                        if errs.gaps.len()>0
                        {
                            for gap in errs.gaps
                            {
                                retval.push(format!("Coverage gap: {site}, {subspecialty}, {context}, {modality}, {weekday}: {}-{}",
                                gap.0,gap.1));
                            }
                        }
                        if errs.overlaps.len()>0
                        {
                            for overlap in errs.overlaps
                            {
                                retval.push(format!("Coverage overlap: {site}, {subspecialty}, {context}, {modality}, {weekday}: {} and {} have overlapping coverage",
                                overlap.0,overlap.1));
                            }
                        }
                    }
                }
            }
        }
    }
    retval
   }
}
impl <'a> CoordinateMap<'a,String,SubspecialtyMap> for CoverageMap
{
    fn get_map(&mut self)->&mut HashMap<String,SubspecialtyMap> {
        &mut self.map
    }
    fn get_coordinate(coords:&CoverageCoordinates)->String {
        coords.site.clone()
    }
}
impl WorkCoverageMap for CoverageMap{
    fn add_work(&mut self,coords:&CoverageCoordinates,work:WorkUnit){
        self.get_branch(coords).add_work(coords,work);
    }
    fn add_coverage(&mut self,coords:&CoverageCoordinates,coverage:CoverageUnit){
        self.get_branch(coords).add_coverage(coords, coverage);
    }
}