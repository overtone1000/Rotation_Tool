
use std::collections::{HashMap, HashSet};

use chrono::NaiveDate;

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
    modality:String
}

impl MapCoords{
}

struct RVUMap
{
    map:HashMap<String,HashMap<String,HashMap<String,HashMap<String,MapEntry>>>>,
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

        for site in SITES
        {
            let mut sub_map:HashMap<String,HashMap<String,HashMap<String,MapEntry>>> = HashMap::new();
            for subspecialty in SUBSPECIALTIES
            {
                let mut con_map:HashMap<String,HashMap<String,MapEntry>> = HashMap::new();
                for context in CONTEXTS
                {
                    let mut mod_map:HashMap<String,MapEntry> = HashMap::new();
                    for modality in MODALITIES
                    {
                        mod_map.insert(modality.to_string(),MapEntry{rvus:0.0});
                    }
                    con_map.insert(context.to_string(),mod_map);
                }
                sub_map.insert(subspecialty.to_string(),con_map);
            }
            retval.map.insert(site.to_string(), sub_map);
        }

        return retval;
    }

    fn getEntry(&self,coords:MapCoords)->Option<&mut MapEntry>
    {
        match self.map.get(&coords.site)?.get(&coords.subspecialty)?.get(&coords.context)?.get(&coords.modality)
        {
            None=>None,
            Some(me)=>Some<
        }
        return retval;
    }
}


pub fn createMap(main_data_table:&table::Table, exam_to_subspecialty_map:&HashMap<String,String>, location_to_context_map:&HashMap<String,String>)->Option<RVUMap>
{
    let mut rvumap = RVUMap::new();
    
    for row_i in main_data_table.rowIndices()
    {
        let datetimestring=main_data_table.getVal(&main_headers::pertinent_headers::scheduled_datetime.getLabel(), &row_i)?;
        
        let date=match NaiveDate::parse_from_str(&datetimestring, "%m/%d/%y %H:%M")
        {
            Ok(dt)=>{
                println!("{},{:?}",datetimestring,dt);
                dt
            },
            Err(e)=>{
                println!("Bad date {:?}",e);
                return None;
            }
        };

        if dates::checkWeekDay(date) && !dates::checkHoliday(date)
        {
            rvumap.included_dates.insert(date);

            let mut coords=MapCoords::default();

            //Trust location and exam code
            let location=main_data_table.getVal(&main_headers::pertinent_headers::location.getLabel(), &row_i)?;
            let exam_code=main_data_table.getVal(&main_headers::pertinent_headers::exam.getLabel(), &row_i)?;

            //Get subspecialty from exam code
            coords.subspecialty=exam_to_subspecialty_map.get(&exam_code)?.to_string();

            //Try context. If not valid, go by site map.
            coords.context= match location_to_context_map.get(&location)
            {
                Some(x)=>x.to_string(),
                None=>location_to_context_map.get(&location)?.to_string()
            };

            //Try site. If not valid, go by location.
            let mut selected_site:Option<String>=None;
            let listed_site=main_data_table.getVal(&main_headers::pertinent_headers::accession.getLabel(), &row_i)?;
            for site in SITES
            {
                if (listed_site[0..site.len()])==site.to_string()
                {
                    selected_site=Some(site.to_string());
                    break;
                }
            }
            if selected_site.is_none()
            {
                selected_site=crate::globals::getLocationSiteMapping(location);  
            }
            coords.site=selected_site?;

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
            match(selected_modality)
            {
                None=>{selected_modality=Some(crate::globals::getModalityAlias(&listed_modality)?.to_string())},
                _=>{}
            }
            coords.modality=selected_modality?;
                        
            let me: Option<&MapEntry>=rvumap.getEntry(coords);
            
            let rvus_str = main_data_table.getVal(&main_headers::pertinent_headers::rvu.getLabel(), &row_i)?;
            let rvus_parse=rvus_str.parse::<f32>();

            match rvus_parse
            {
                Ok(rvus)=>{
                    match me
                    {
                        None=>{return None;},
                        Some(me_x)=>{me_x.addRVUs(rvus);}
                    }
                }
                Err(x)=>{
                    println!("Error: {}", x);
                    return None;
                }
            }
        }
    }

    Some(rvumap)
}