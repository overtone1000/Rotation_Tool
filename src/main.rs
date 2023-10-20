use std::{error::Error, fs::{self, File}, collections::HashMap, io::Write};

use chrono::{DateTime, Local};
use globals::main_headers;


use crate::{globals::file_names, error::RotationToolError, categorization::{buildSalemRVUMap, get_categories_list, get_locations_list, backup, buildSalemBVUMap}, time::getTimeRowNormalDistWeights};

mod globals;
mod error;
mod time;
mod table;
mod dates;
mod rvu_map;
mod tpc;
mod categorization;

fn main()->Result<(), Box<dyn Error>> {
    let main_data_table=table::Table::create(file_names::MAIN_DATA_FILE)?;
    let tpc_data_table=table::Table::create(file_names::TPC_DATA_FILE)?;
    let bvu_data_table=table::Table::create(file_names::BVU_DATA_FILE)?;

    //This was only necessary to fix the data.
    //let mut mut_bvu_data_table=table::Table::create(file_names::BVU_DATA_FILE)?;
    //crate::categorization::checkBVUSource(&main_data_table, &mut mut_bvu_data_table);

    //Get current categories
    let mut exam_categories_table=table::Table::create(file_names::CATEGORIES_EXAM_FILE)?;
    let mut location_categories_table=table::Table::create(file_names::CATEGORIES_LOCATION_FILE)?;

    let exam_categories_list = get_categories_list(&main_data_table,&exam_categories_table)?;
    
    exam_categories_table.clear();
    for category_row in exam_categories_list.as_slice()
    {
        let mut newrow:Vec<String>=Vec::new();
        newrow.push(category_row.procedure_code.to_owned());
        newrow.push(category_row.exam.to_owned());
        newrow.push(category_row.subspecialty.to_owned());
        newrow.push(category_row.comments.to_owned());
        exam_categories_table.pushrow(newrow);
    }

   
    let location_categories_list = get_locations_list(&main_data_table,&location_categories_table)?;
    
    location_categories_table.clear();
    for location_row in location_categories_list.as_slice()
    {
        let mut newrow:Vec<String>=Vec::new();
        newrow.push(location_row.location.to_owned());
        newrow.push(location_row.context.to_owned());
        newrow.push(location_row.comments.to_owned());
        location_categories_table.pushrow(newrow);
    }
        
    let dt = chrono::offset::Local::now();

    //Archive and save new file if changed
    match backup(dt,file_names::CATEGORIES_EXAM_FILE.to_string(),"Categories_Exam.csv".to_owned())
    {
        Ok(_)=>{
            exam_categories_table.write_to_file(file_names::CATEGORIES_EXAM_FILE.to_owned());
        },
        Err(x)=>{
            println!("{}",x);
            return Err(Box::new(x));
        }
    }
    match backup(dt,file_names::CATEGORIES_LOCATION_FILE.to_string(),"Categories_Location.csv".to_owned())
    {
        Ok(_)=>{
            location_categories_table.write_to_file(file_names::CATEGORIES_LOCATION_FILE.to_owned());
        },
        Err(x)=>{
            println!("{}",x);
            return Err(Box::new(x));
        }
    }

    let mut exam_to_subspecialty_map:HashMap<String,String>=HashMap::new();
    let mut location_to_context_map:HashMap<String,String>=HashMap::new();

    for exam_category in exam_categories_list
    {
        exam_to_subspecialty_map.insert(exam_category.procedure_code,exam_category.subspecialty);
    }

    for location_category in location_categories_list
    {
        location_to_context_map.insert(location_category.location,location_category.context);
    }

    //Create the conventional RVU map
    {
        let rvu_map=buildSalemRVUMap(&main_data_table)?;
        let map = match rvu_map::createMap(&main_data_table,&tpc_data_table,&rvu_map,&exam_to_subspecialty_map,&location_to_context_map)
        {
            Ok(x)=>x,
            Err(e)=>{
                let err=RotationToolError::new(e);
                return Err(Box::new(err));
            }
        };

        match map.toFile(file_names::OUT_FILE)
        {
            Ok(_)=>{},
            Err(e)=>{return Err(e);}
        }
    }

    //Create BVU map
    {
        let bvu_map: HashMap<String, f32>=buildSalemBVUMap(&bvu_data_table)?;

        let map = match rvu_map::createMap(&main_data_table,&tpc_data_table,&bvu_map,&exam_to_subspecialty_map,&location_to_context_map)
        {
            Ok(x)=>x,
            Err(e)=>{
                let err=RotationToolError::new(e);
                return Err(Box::new(err));
            }
        };
        
        match map.toFile(file_names::BVU_OUT_FILE)
        {
            Ok(_)=>{},
            Err(e)=>{return Err(e);}
        }
    }
    
    println!("Finished.");
    return Ok(());
}

