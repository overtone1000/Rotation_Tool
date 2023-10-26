
use std::{collections::{HashMap, HashSet}, hash::Hash, error::Error, fs::File, io::Write, convert::Infallible};

use chrono::{NaiveDate, NaiveDateTime, Timelike};

use crate::{table::{Table, self}, globals::{SITES, SUBSPECIALTIES, CONTEXTS, MODALITIES, main_headers, tpc_headers}, dates::{self, business_days_per_year}, tpc, ProcessedSource};

struct MapEntry
{
    rvus:f64
}

impl MapEntry
{
    fn addRVUs(&mut self,rvus:f64)
    {
        self.rvus+=rvus;
    }

    fn getRVUs(&self)->f64
    {
        return self.rvus.to_owned();
    }

    fn setRVUs(&mut self,rvu:f64)->()
    {
        self.rvus=rvu;
    }
}

#[derive(Default)]
pub struct MapCoords
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
    
    pub fn getSubspecialty(&self)->&String{return &self.subspecialty}
    pub fn getContext(&self)->&String{return &self.context}
    pub fn getSite(&self)->&String{return &self.site}
}

pub struct RVUMap
{
    //site, subspecialty, context, modality, time_row
    map:HashMap<String,HashMap<String,HashMap<String,HashMap<String,HashMap<usize,MapEntry>>>>>
}

impl RVUMap
{
    fn new()->RVUMap
    {
        let mut retval = RVUMap{
            map:HashMap::new()
        };

        return retval;
    }

    fn addRVUs(&mut self,coords:&MapCoords,rvus:f64)->Result<String,String>
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
                                                modalitynode[time_row.to_string()]=me.rvus.into();
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

    pub fn toFile(&self, filename:&str)->Result<(), Box<dyn Error>>{
        let mut mapoutfile=File::create(filename)?;
        let mapstr=self.toJSON()?;
        let bytes=mapstr.as_bytes();
            
        return match mapoutfile.write_all(&bytes){
            Ok(_)=>{Ok(())},
            Err(e)=>{return Err(Box::new(crate::RotationToolError::new(e.to_string())));}
        }
    }

    pub fn totalAverageRVUs(&self)->f64
    {
        self.sliceAverageRVUs(None,None)
    }

    pub fn sliceAverageRVUs(&self, inclusion_function:Option<fn(&MapCoords)->bool>,exclusion_function:Option<fn(&MapCoords)->bool>)->f64
    {
        //site, subspecialty, context, modality, time_row
        let mut retval:f64=0.0;
        for (site, m1) in &self.map
        {
            for (subspecialty, m2) in m1
            {
                for (context, m3) in m2
                {
                    for (modality, m4) in m3
                    {
                        for (time_row, me) in m4
                        {
                            let coords = MapCoords{
                                site: site.to_owned(),
                                subspecialty: subspecialty.to_owned(),
                                context: context.to_owned(),
                                modality: modality.to_owned(),
                                time_row: time_row.to_owned(),
                            };

                            let include = match inclusion_function
                            {
                                Some(inclusion_function)=>inclusion_function(&coords),
                                None=>true
                            };

                            let exclude = match exclusion_function
                            {
                                Some(exclusion_function)=>exclusion_function(&coords),
                                None=>false
                            };

                            if include && !exclude
                            {
                                if(me.rvus.is_infinite())
                                {
                                    eprintln!("Infinite RVUs!");
                                }
                                retval+=me.rvus;
                                if(retval.is_infinite())
                                {
                                    eprintln!("Infinite retval!");
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

pub fn createMap(source:&ProcessedSource, rvu_map:&HashMap<String,f64>, include_date:fn(NaiveDateTime)->bool)->Result<RVUMap,String>
{
    let mut rvumap = RVUMap::new();

    let mut modality_map:HashMap<String,String>=HashMap::new();

    let mut included_dates:HashSet<NaiveDate>=HashSet::new();
    
    for row_i in source.main_data_table.rowIndices()
    {
        let datetimestring= source.main_data_table.getVal(&main_headers::pertinent_headers::scheduled_datetime.getLabel(), &row_i)?;
        
        let datetime=match NaiveDateTime::parse_from_str(&datetimestring, "%m/%d/%y %H:%M"){
            Ok(x)=>x,
            Err(x)=>{return Err(format!("Couldn't parse date {}",datetimestring));}
        };

        let location=source.main_data_table.getVal(&main_headers::pertinent_headers::location.getLabel(), &row_i)?;
        let exam_code=source.main_data_table.getVal(&main_headers::pertinent_headers::procedure_code.getLabel(), &row_i)?;

        //Build coords and populate maps with this row.
        let mut coords=MapCoords::default();
        {
            coords.time_row=crate::time::getTimeRowIndex(datetime.hour(),datetime.minute());

            //Get subspecialty from exam code
            coords.subspecialty=match source.exam_to_subspecialty_map.get(&exam_code)
            {
                Some(x)=>x.to_string(),
                None=>{
                    return Err(format!("Invalid exam_code {} in exam_to_subspeciality_map",exam_code));
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
            coords.site=match selected_site
            {
                Some(x)=>x,
                None=>{
                    return Err(format!("Could not determine site for row {}",row_i));
                }
            };

            //Try context. If not valid, go by site map.
            coords.context= match source.location_to_context_map.get(&location)
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
            coords.modality=match selected_modality
            {
                Some(x)=>x,
                None=>{
                    return Err(format!("Could not determine modality for row {}",row_i));
                }
            };
            if(!modality_map.contains_key(&exam_code))
            {
                modality_map.insert(exam_code.to_owned(), coords.modality.to_owned());
            }
        }

        //Check if this date should be included in RVU totals. If so, add rvus.
        if include_date(datetime)
        {
            included_dates.insert(NaiveDate::from(datetime));

            //let rvus_str = main_data_table.getVal(&main_headers::pertinent_headers::rvu.getLabel(), &row_i)?;
            let rvus=match rvu_map.get(&exam_code){
                Some(&x)=>x,
                None=>{
                    return Err(format!("Coudn't find exam code {}",exam_code));
                }
            };

            rvumap.addRVUs(&coords,rvus)?;
        }
    }   

    let days:f64=included_dates.len() as f64;

    if days==0.0
    {
        eprintln!("Zero days!!");
    }

    //Divide by number of days worth of data to get rvu/day
    for site in rvumap.map.iter_mut()
    {
        let sub_map = site.1;
        for subspecialty in sub_map.iter_mut()
        {
            let con_map=subspecialty.1;
            for context in con_map.iter_mut()
            {
                let mod_map=context.1;
                for modality in mod_map.iter_mut()
                {
                    let time_map=modality.1;
                    for time_row in time_map.iter_mut()
                    {
                        let me = time_row.1;
                        me.setRVUs(me.rvus/days);
                    }
                }
            }
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
            Err(e)=>{return Err(format!("{:?}",e));}
        };

        let number_per_business_day=number/business_days_per_year;
        let rvus_per_exam=match rvu_map.get(&exam_code){
            None=>{return Err(format!("Bad exam code {}",exam_code));}
            Some(val)=>val.to_owned()
        };

        let rvus_per_business_day =number_per_business_day*rvus_per_exam;

        let mut coords=MapCoords::default();
        coords.site=crate::globals::TPC.to_string();
        coords.subspecialty=match source.exam_to_subspecialty_map.get(&exam_code){
            None=>{return Err(format!("Bad exam code {}",exam_code));}
            Some(val)=>val.to_owned()
        };
        coords.context=crate::globals::Outpatient.to_string();
        coords.modality=match modality_map.get(&exam_code)
        {
            None=>{return Err(format!("Bad exam code {}",exam_code));}
            Some(val)=>val.to_owned()
        };

        for key in weights.keys() {
            coords.time_row=*key;
            let rvu=rvus_per_business_day*(*weights.get(key).expect("Expected")) as f64;
            rvumap.addRVUs(&coords, rvu);
        }
    }

    Ok(rvumap)
}