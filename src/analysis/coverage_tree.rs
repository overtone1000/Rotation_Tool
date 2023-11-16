use std::collections::HashMap;

use chrono::{NaiveTime, NaiveDateTime, Datelike, NaiveDate, Duration, Days};

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

#[derive(Default,Debug)]
pub struct CoverageUnit
{
    rotation:String,
    start:NaiveTime,
    end:NaiveTime
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

pub trait WorkCoverageMap
{
    fn add_work(&self,coords:&CoverageCoordinates,work:WorkUnit);
    fn add_coverage(&self,coords:&CoverageCoordinates,coverage:CoverageUnit);
}
pub trait CoordinateMap<T,U> 
    where T:std::fmt::Debug,T:Eq,T:PartialEq,T:std::hash::Hash,U:Default,U:std::fmt::Debug
{
    fn get_map(&self)->HashMap<T,U>;
    fn get_coordinate(coords:&CoverageCoordinates)->&T;
    fn get_branch(&mut self, coords:&CoverageCoordinates)->&mut U
    {
        match self.get_map().get_mut(Self::get_coordinate(coords))
        {
            Some(x)=>{
                x
            }
            None=>{
                let mut newbranch=U::default();
                self.get_map().try_insert(*Self::get_coordinate(coords),newbranch).expect("Checked")
            }
        }
    }
}

pub trait ParentCoordinateMap<T,U>:CoordinateMap<T,U>+WorkCoverageMap
    where T:std::fmt::Debug,T:Eq,T:PartialEq,T:std::hash::Hash,U:Default,U:std::fmt::Debug,
    U:WorkCoverageMap
{
    fn add_work(&self,coords:&CoverageCoordinates,work:WorkUnit){
        self.get_branch(coords).add_work(coords,work);
    }
    fn add_coverage(&self,coords:&CoverageCoordinates,coverage:CoverageUnit){
        self.get_branch(coords).add_coverage(coords, coverage);
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
    fn add_work(&self,coords:&CoverageCoordinates,work:WorkUnit){
        self.get_branch(coords).work.push(work);
    }
    fn add_coverage(&self,coords:&CoverageCoordinates,coverage:CoverageUnit){
        self.get_branch(coords).coverages.push(coverage);
    }
}
impl CoordinateMap<chrono::Weekday,CoverageAndWorkDay> for WeekdayMap
{
    fn get_map(&self)->HashMap<chrono::Weekday,CoverageAndWorkDay> {
        self.map
    }
    fn get_coordinate(coords:&CoverageCoordinates)->&chrono::Weekday {
        &coords.weekday
    }
}

#[derive(Default, Debug)]
pub struct ContextMap {
    map:HashMap<String, WeekdayMap>
}
impl ContextMap
{
    fn add_work(&mut self, coords:&CoverageCoordinates, work:WorkUnit){
        
    }
}
impl CoordinateMap<String,WeekdayMap> for ContextMap
{
    fn get_map(&self)->HashMap<String,WeekdayMap> {
        self.map
    }
    fn get_coordinate(coords:&CoverageCoordinates)->&String {
        &coords.context
    }
}
impl ParentCoordinateMap<String,WeekdayMap> for ContextMap{}

#[derive(Default,Debug)]
pub struct SubspecialtyMap {
     map:HashMap<String, ContextMap>
}
impl CoordinateMap<String,ContextMap> for SubspecialtyMap
{
    fn get_map(&self)->HashMap<String,ContextMap> {
        self.map
    }
    fn get_coordinate(coords:&CoverageCoordinates)->&String {
        &coords.subspecialty
    }
}
impl ParentCoordinateMap<String,ContextMap> for SubspecialtyMap{}

#[derive(Default)]
pub struct CoverageTree {
    map:HashMap<String, SubspecialtyMap>
}
impl CoverageTree
{
   fn add_work(&mut self, coords:&CoverageCoordinates, work:WorkUnit){
    
   }

   pub fn build<'a>(source:ProcessedSource, date_constraints:&ConstraintSet<'a,NaiveDateTime>)->Result<CoverageTree,Box<dyn std::error::Error>>
   {

    let mut retval=CoverageTree::default();

    let mut modality_map:HashMap<String,String>=HashMap::new();

    let exam_rvu_map=buildSalemRVUMap(&source.main_data_table)?;
    let exam_bvu_map: HashMap<String, f64>=buildSalemBVUMap(&source.bvu_data_table)?;
    
    for row_i in source.main_data_table.rowIndices()
    {
        let datetimestring= source.main_data_table.getVal(&main_headers::pertinent_headers::scheduled_datetime.getLabel(), &row_i)?;
        
        let datetime=match NaiveDateTime::parse_from_str(&datetimestring, "%m/%d/%y %H:%M"){
            Ok(x)=>x,
            Err(x)=>{return Err(Box::new(x));}
        };

        if date_constraints.include(&datetime)
        {

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
                    rvu:*rvu,
                    bvu:*bvu
                }
            };

            retval.add_work(&coords,work);
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
                retval.add_work(&coords,work);
            }
        }        
    }

    Ok(retval)

   }
}
impl CoordinateMap<String,SubspecialtyMap> for CoverageTree
{
    fn get_map(&self)->HashMap<String,SubspecialtyMap> {
        self.map
    }
    fn get_coordinate(coords:&CoverageCoordinates)->&String {
        &coords.site
    }
}
impl ParentCoordinateMap<String,SubspecialtyMap> for CoverageTree{}