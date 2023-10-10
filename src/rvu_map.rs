
use std::{collections::{HashMap, HashSet}, hash::Hash};

use chrono::{NaiveDate, NaiveDateTime, Timelike};

use crate::{table::{Table, self}, globals::{SITES, SUBSPECIALTIES, CONTEXTS, MODALITIES, main_headers}, dates};

struct MapEntry
{
    rvus:f32
}

impl MapEntry
{
    fn addRVUs(&mut self,rvus:f32)
    {
        self.rvus+=rvus;
    }

    fn getRVUs(&self)->f32
    {
        return self.rvus.to_owned();
    }
}

#[derive(Default)]
struct MapCoords
{
    site:String,
    subspecialty:String,
    context:String,
    modality:String,
    time_row:usize
}

impl MapCoords{
    fn validate(s:String,list:&[&str])->bool
    {
        for member in list
        {
            if(member.to_string()==s)
            {
                return true;
            }
        }
        return false;
    }
    pub fn validateSite(&self)->bool
    {
        let retval = MapCoords::validate(self.site.to_owned(),SITES);
        if(!retval){
            eprintln!("Invalid site {}",self.site);
        }
        return retval;
    }
    pub fn validateSubspecialty(&self)->bool
    {
        let retval =  MapCoords::validate(self.subspecialty.to_owned(),SUBSPECIALTIES);
        if(!retval){
            eprintln!("Invalid subspecialty {}",self.subspecialty);
        }
        return retval;
    }
    pub fn validateContext(&self)->bool
    {
        let retval =  MapCoords::validate(self.context.to_owned(),CONTEXTS);
        if(!retval){
            eprintln!("Invalid context {}",self.context);
        }
        return retval;
    }
    pub fn validateModality(&self)->bool
    {
        let retval =  MapCoords::validate(self.modality.to_owned(),MODALITIES);
        if(!retval){
            eprintln!("Invalid modality {}",self.modality);
        }
        return retval;
    }
}

pub struct RVUMap
{
    map:HashMap<String,HashMap<String,HashMap<String,HashMap<String,HashMap<usize,MapEntry>>>>>,
    included_dates:HashSet<NaiveDate>
}

impl RVUMap
{
    fn new()->RVUMap
    {
        let mut retval = RVUMap{
            map:HashMap::new(),
            included_dates:HashSet::new()
        };

        return retval;
    }

    fn addRVUs(&mut self,coords:&MapCoords,rvus:f32)->Result<String,String>
    {
        if(!coords.validateSite()){return Err("Invalid site.".to_string());}
        if(!self.map.contains_key(&coords.site))
        {
            let map=HashMap::new();
            self.map.insert(coords.site.to_owned(),map);
        }       
        let sub_map=self.map.get_mut(&coords.site).expect("Immediate get");

        if(!coords.validateSubspecialty()){return  Err("Invalid subspecialty.".to_string());}
        if(!sub_map.contains_key(&coords.subspecialty))
        {
            let map=HashMap::new();
            sub_map.insert(coords.subspecialty.to_owned(),map);
        }       
        let con_map=sub_map.get_mut(&coords.subspecialty).expect("Immediate get");

        if(!coords.validateContext()){return  Err("Invalid context.".to_string());}
        if(!con_map.contains_key(&coords.context))
        {
            let map=HashMap::new();
            con_map.insert(coords.context.to_owned(),map);
        }       
        let mod_map=con_map.get_mut(&coords.context).expect("Immediate get");

        if(!coords.validateModality()){return  Err("Invalid modality.".to_string());}
        if(!mod_map.contains_key(&coords.modality))
        {
            let map=HashMap::new();
            mod_map.insert(coords.modality.to_owned(),map);
        }       
        let time_map=mod_map.get_mut(&coords.modality).expect("Immediate get");

        if(!time_map.contains_key(&coords.time_row))
        {
            let map_entry:MapEntry=MapEntry{rvus:0.0};
            time_map.insert(coords.time_row,map_entry);
        }
        let me= time_map.get_mut(&coords.time_row).expect("Immediate get");
        me.addRVUs(rvus);
        return Ok("good".to_string());

    }

    pub fn toJSON(&self)->Result<String,String>
    {
        let days:f32=self.included_dates.len() as f32;

        let mut topnode=json::JsonValue::new_object();
        if(self.map.keys().len()>0)
        {
            for site in self.map.keys()
            {
                let sub_map = self.map.get(site).expect("No submap");
                if(sub_map.keys().len()>0)
                {
                    let mut sitenode: json::JsonValue=json::JsonValue::new_object();
                    for subspecialty in sub_map.keys()
                    {
                        let con_map=sub_map.get(subspecialty).expect("No conmap");
                        if(con_map.keys().len()>0)
                        {
                            let mut subspecialtynode=json::JsonValue::new_object();
                            for context in con_map.keys()
                            {
                                let mod_map=con_map.get(context).expect("No modmap");
                                if(mod_map.keys().len()>0)
                                {
                                    let mut contextnode=json::JsonValue::new_object();
                                    for modality in mod_map.keys()
                                    {
                                        let time_map=mod_map.get(modality).expect("No time map");
                                        if(time_map.keys().len()>0)
                                        {
                                            let mut modalitynode=json::JsonValue::new_object();
                                            for time_row in time_map.keys()
                                            {
                                                let me = time_map.get(time_row).expect("No map entry");
                                                let avg_rvus_per_day=me.rvus/days;
                                                modalitynode[time_row.to_string()]=avg_rvus_per_day.into();
                                            }
                                            contextnode[modality]=modalitynode;
                                        }
                                    }
                                    subspecialtynode[context]=contextnode;
                                }
                            }
                            sitenode[subspecialty]=subspecialtynode;
                        }
                    }
                    topnode[site]=sitenode;
                }
            }
        }

        Ok(topnode.dump())
    }
}


pub fn createMap(main_data_table:&table::Table, exam_to_subspecialty_map:&HashMap<String,String>, location_to_context_map:&HashMap<String,String>)->Result<RVUMap,String>
{
    let mut rvumap = RVUMap::new();
    
    for row_i in main_data_table.rowIndices()
    {
        let datetimestring= main_data_table.getVal(&main_headers::pertinent_headers::scheduled_datetime.getLabel(), &row_i)?;
        
        let datetime=match NaiveDateTime::parse_from_str(&datetimestring, "%m/%d/%y %H:%M"){
            Ok(x)=>x,
            Err(x)=>{return Err(format!("Couldn't parse date {}",datetimestring));}
        };

        let date=NaiveDate::from(datetime);

        if dates::checkWeekDay(date) && !dates::checkHoliday(date)
        {
            rvumap.included_dates.insert(date);

            let mut coords=MapCoords::default();

            coords.time_row=crate::time::getTimeRowIndex(datetime.hour(),datetime.minute());

            //Trust location and exam code
            let location=main_data_table.getVal(&main_headers::pertinent_headers::location.getLabel(), &row_i)?;
            let exam_code=main_data_table.getVal(&main_headers::pertinent_headers::procedure_code.getLabel(), &row_i)?;

            //Get subspecialty from exam code
            coords.subspecialty=match exam_to_subspecialty_map.get(&exam_code)
            {
                Some(x)=>x.to_string(),
                None=>{
                    return Err(format!("Invalid exam_code {} in exam_to_subspeciality_map",exam_code));
                }
            };


            //Try site. If not valid, go by location.
            let mut selected_site:Option<String>=None;
            let listed_site=main_data_table.getVal(&main_headers::pertinent_headers::accession.getLabel(), &row_i)?;
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
            coords.site=match selected_site
            {
                Some(x)=>x,
                None=>{
                    return Err(format!("Could not determine site for row {}",row_i));
                }
            };

            //Try context. If not valid, go by site map.
            coords.context= match location_to_context_map.get(&location)
            {
                Some(x)=>x.to_string(),
                None=>{    
                    match crate::globals::getLocationSiteMapping(&location)
                    {
                        Some(x)=>x,
                        None=>{
                            return Err(format!("Could not determine context for location {}",location));
                        }
                    }
                }
            };          

            //Get modality, but check for aliases
            let listed_modality = main_data_table.getVal(&main_headers::pertinent_headers::modality.getLabel(), &row_i)?;
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
                    selected_modality=crate::globals::getModalityFromProcedureDesc(main_data_table.getVal(&main_headers::pertinent_headers::exam.getLabel(), &row_i)?)
                },
                _=>{}
            }
            coords.modality=match selected_modality
            {
                Some(x)=>x,
                None=>{
                    return Err(format!("Could not determine modality for row {}",row_i));
                }
            };

            let rvus_str = main_data_table.getVal(&main_headers::pertinent_headers::rvu.getLabel(), &row_i)?;
            let rvus=match rvus_str.parse::<f32>()
            {
                Ok(x)=>x,
                Err(e)=>{
                    return Err(format!("{:?}",e));
                }
            };

            rvumap.addRVUs(&coords,rvus)?;
        }
    }   

    Ok(rvumap)
}