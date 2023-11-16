use std::collections::HashMap;

use chrono::{NaiveTime, NaiveDateTime, Datelike};

use crate::{processed_source::ProcessedSource, globals::{main_headers, SITES, MODALITIES, tpc_headers}, constraints::ConstraintSet, dates::business_days_per_year};

use super::source_error::SourceError;

pub struct CoverageCoordinates
{
    site:String,
    subspecialty:String,
    context:String,
    weekday:chrono::Weekday
}

#[derive(Default)]
pub struct CoverageUnit
{
    rotation:String,
    start:NaiveTime,
    end:NaiveTime
}

#[derive(Default)]
pub struct WorkUnit
{
    datetime:NaiveDateTime,
    rvu:f64,
    bvu:f64
}

#[derive(Default)]
pub struct CoverageAndWorkDay
{
    coverages:Vec<CoverageUnit>,
    work:Vec<WorkUnit>
}

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
    fn get_weekday(&mut self, wd:chrono::Weekday)->Option<&mut CoverageAndWorkDay>{self.map.get_mut(&wd)}
}

#[derive(Default)]
pub struct ContextMap {
    map:HashMap<String, WeekdayMap>
}
impl ContextMap
{
    fn get_context(&mut self, context:&String)->Option<&mut WeekdayMap>{self.map.get_mut(context)}

    fn add_leaf(&mut self, coords:&CoverageCoordinates, work:WorkUnit){
        match self.get_context(&coords.context)
        {
            None=>{
                let newbranch=WeekdayMap::default();
                self.map.insert(coords.context.to_string(),newbranch);
            }
            _=>()
        }
    }
}

#[derive(Default)]
pub struct SubspecialtyMap {
     map:HashMap<String, ContextMap>
}
impl SubspecialtyMap
{
    fn get_subspecialty(&mut self, subspecialty:&String)->Option<&mut ContextMap>{self.map.get_mut(subspecialty)}

    fn add_leaf(&mut self, coords:&CoverageCoordinates, work:WorkUnit){
        match self.get_subspecialty(&coords.subspecialty)
        {
            Some(x)=>{
                x.add_leaf(coords,work)
            }
            None=>{
                let mut newbranch=ContextMap::default();
                newbranch.add_leaf(coords,work);
                self.map.insert(coords.subspecialty.to_string(),newbranch);
            }
        }
    }
}

#[derive(Default)]
pub struct CoverageTree {
    map:HashMap<String, SubspecialtyMap>
}
impl CoverageTree
{
   fn get_site(&mut self, site:&String)->Option<&mut SubspecialtyMap>{self.map.get_mut(site)}

   fn add_leaf(&mut self, coords:&CoverageCoordinates, work:WorkUnit){
    match self.get_site(&coords.site)
    {
        Some(x)=>{
            x.add_leaf(coords,work)
        }
        None=>{
            let mut newbranch=SubspecialtyMap::default();
            newbranch.add_leaf(coords,work);
            self.map.insert(coords.site.to_string(),newbranch);
        }
    }
   }

   fn build<'a>(&self, source:ProcessedSource, exam_rvu_map:&HashMap<String,f64>, date_constraints:&ConstraintSet<'a,NaiveDateTime>)->Result<CoverageTree,Box<dyn std::error::Error>>
   {

    let retval=CoverageTree::default();

    let mut modality_map:HashMap<String,String>=HashMap::new();
    
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
                    modality_map.insert(exam_code.to_owned(), modality);
                }

                CoverageCoordinates{
                    site:site,
                    subspecialty:subspecialty,
                    context:context,
                    weekday:datetime.weekday(),
                }
            };

            let work:WorkUnit =
            {
                let rvu = exam_rvu_map.get()
                WorkUnit {
                    datetime:datetime,
                    
                }
            };

            self.add_leaf(&coords,work);
        }
    }
    //Add TPC, which doesn't go by number of dates
    let weights=crate::time::getTimeRowNormalDistWeights();
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
            None=>{SourceError::generate(format!("Bad exam code {}",exam_code));}
            Some(val)=>val.to_owned()
        };

        let rvus_per_business_day =number_per_business_day*rvus_per_exam;

        let mut coords=CoverageCoordinates::default();
        coords.site=crate::globals::TPC.to_string();
        coords.subspecialty=match source.exam_to_subspecialty_map.get(&exam_code){
            None=>{SourceError::generate(format!("Bad exam code {}",exam_code));}
            Some(val)=>val.to_owned()
        };
        coords.context=crate::globals::Outpatient.to_string();
        coords.modality=match modality_map.get(&exam_code)
        {
            None=>{SourceError::generate(format!("Bad exam code {}",exam_code));}
            Some(val)=>val.to_owned()
        };

        for key in weights.keys() {
            coords.time_row=*key;
            let rvu=rvus_per_business_day*(*weights.get(key).expect("Expected")) as f64;
            rvumap.addRVUs(&coords, rvu);
        }
    }

    retval

   }
}