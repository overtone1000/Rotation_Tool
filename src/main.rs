#![allow(unused_parens)]

use std::{error::Error, fs::{self, File}, collections::HashMap, io::Write};

use chrono::{DateTime, Local};
use globals::main_headers;


use crate::{globals::file_names, error::RotationToolError, categorization::{buildSalemRVUMap, get_categories_list, get_locations_list, backup}, time::getTimeRowNormalDistWeights};

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

    let rvu_map=buildSalemRVUMap(&main_data_table)?;

    let tpc_data_table=table::Table::create(file_names::TPC_DATA_FILE)?;

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

    let map = match rvu_map::createMap(&main_data_table,&tpc_data_table,&rvu_map,&exam_to_subspecialty_map,&location_to_context_map)
    {
        Ok(x)=>x,
        Err(e)=>{
            let err=RotationToolError::new(e);
            return Err(Box::new(err));
        }
    };

    let mut mapoutfile=File::create(file_names::OUT_FILE)?;
    let mapstr=map.toJSON()?;
    let bytes=mapstr.as_bytes();
        
    match mapoutfile.write_all(&bytes){
        Ok(_)=>{},
        Err(e)=>{return Err(Box::new(RotationToolError::new(e.to_string())));}
    }
    
    println!("Finished.");
    return Ok(());
}
