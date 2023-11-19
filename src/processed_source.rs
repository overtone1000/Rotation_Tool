use std::{collections::HashMap, error::Error, fs::File, io::{BufWriter, BufReader}};

use serde::{Deserialize, Serialize};

use crate::{table::{Table, self}, categorization::{exam_categories::exam_category, get_categories_list, get_locations_list, backup}, globals::file_names};

#[derive(Serialize,Deserialize)]
pub struct ProcessedSource
{
    pub main_data_table:Table,
    pub tpc_data_table:Table,
    pub bvu_data_table:Table,
    pub exam_categories_table:Table,
    pub location_categories_table:Table,
    pub exam_categories_list:Vec<exam_category>,
    pub exam_to_subspecialty_map:HashMap<String,String>,
    pub location_to_context_map:HashMap<String,String>
}

impl ProcessedSource
{
    pub fn build()->Result<ProcessedSource, Box<dyn Error>>{
        let main_data_table= table::Table::create(file_names::MAIN_DATA_FILE)?;
        let tpc_data_table= table::Table::create(file_names::TPC_DATA_FILE)?;
        let bvu_data_table= table::Table::create(file_names::BVU_DATA_FILE)?; 
        let mut exam_categories_table= table::Table::create(file_names::CATEGORIES_EXAM_FILE)?;
        let mut location_categories_table= table::Table::create(file_names::CATEGORIES_LOCATION_FILE)?;
        let exam_categories_list=get_categories_list(&main_data_table,&exam_categories_table)?;
        let mut exam_to_subspecialty_map:HashMap<String,String>=HashMap::new();
        let mut location_to_context_map:HashMap<String,String>=HashMap::new();

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
        /*
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
        */

        for exam_category in &exam_categories_list
        {
            exam_to_subspecialty_map.insert(exam_category.procedure_code.to_string(),exam_category.subspecialty.to_string());
        }

        for location_category in location_categories_list
        {
            location_to_context_map.insert(location_category.location,location_category.context);
        }

        Ok(ProcessedSource {
            main_data_table:main_data_table,
            tpc_data_table:tpc_data_table,
            bvu_data_table:bvu_data_table,
            exam_categories_table:exam_categories_table,
            location_categories_table:location_categories_table,
            exam_categories_list: exam_categories_list,
            exam_to_subspecialty_map:exam_to_subspecialty_map,
            location_to_context_map:location_to_context_map,
        })
    }

    pub fn save_to_cache(&self,filename:&str)->Result<(),Box<dyn Error>>
    {
        println!("Saving processed source to cache.");
        let cachefile=File::create(filename)?;
        let bufwriter = BufWriter::new(cachefile);
        let mut serializer = serde_json::Serializer::new(bufwriter);

        Ok(self.serialize(&mut serializer)?)
    }

    pub fn load_from_cache(filename:&str)->Result<ProcessedSource,Box<dyn Error>>
    {
        println!("Reading processed source from cache.");
        let cachefile=File::open(filename)?;
        let bufreader = BufReader::new(cachefile);
        let mut deserializer = serde_json::Deserializer::from_reader(bufreader);
        
        Ok(ProcessedSource::deserialize(&mut deserializer)?)
    }

    pub fn checkBVUSource(&mut self)
    {
        //This was only necessary to fix the data.
        crate::categorization::checkBVUSource(&self.main_data_table, &mut self.bvu_data_table);
    }
}

