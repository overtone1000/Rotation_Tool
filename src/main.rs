#![allow(unused_parens)]

use std::{error::Error, fs::{self, File}, collections::HashMap, io::Write};

use chrono::{DateTime, Local};
use globals::main_headers;


use crate::{globals::file_names, error::RotationToolError};

mod globals;
mod error;
mod time;
mod table;
mod dates;
mod rvu_map;

mod exam_categories {
    use std::cmp::Ordering;

    pub(crate) enum pertinent_headers {
        procedure_code,
        exam,
        subspecialty,
        comments
    }

    impl pertinent_headers {
        pub(crate) fn getLabel(&self)->String
        {
            match self{
                pertinent_headers::procedure_code => "Exam Code".to_string(),
                pertinent_headers::exam => "Exam Description".to_string(),
                pertinent_headers::subspecialty => "Subspecialty".to_string(),
                pertinent_headers::comments => "Comments".to_string(),
            }
        }
    }
    
    pub(crate) struct exam_category {
        pub procedure_code:String,
        pub exam:String,
        pub subspecialty:String,
        pub comments:String
    }

    impl Eq for exam_category {}

    impl PartialOrd for exam_category {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl PartialEq for exam_category {
        fn eq(&self, other: &Self) -> bool {
            self.procedure_code == other.procedure_code &&
            self.exam == other.exam
            //self.subspecialty == other.subspecialty &&
            //self.comments == other.comments
        }
    }

    impl Ord for exam_category {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.exam.cmp(&other.exam)
            {
                std::cmp::Ordering::Equal => self.procedure_code.cmp(&other.procedure_code),
                (examcmp) => examcmp
            }
        }
    }
}

mod location_categories {
    use std::cmp::Ordering;

    pub(crate) enum pertinent_headers {
        location,
        context,
        comments
    }

    impl pertinent_headers {
        pub(crate) fn getLabel(&self)->String
        {
            match self{
                pertinent_headers::location => "Location".to_string(),
                pertinent_headers::context => "Context".to_string(),
                pertinent_headers::comments => "Comments".to_string(),
            }
        }
    }
    
    pub(crate) struct location_category {
        pub location:String,
        pub context:String,
        pub comments:String
    }

    impl Eq for location_category {}

    impl PartialOrd for location_category {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl PartialEq for location_category {
        fn eq(&self, other: &Self) -> bool {
            self.location == other.location
            //self.context == other.context &&
            //self.comments == other.comments
        }
    }

    impl Ord for location_category {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.location.cmp(&other.location)
        }
    }
}

fn get_categories_list(
    main_data_table:&table::Table,
    exam_categories_table:&table::Table
)->Result<Vec<exam_categories::exam_category>,String>
{
    let main_exam_categories= main_data_table.getKeyedColumnSampleMap(
        &(main_headers::pertinent_headers::procedure_code.getLabel())
    )?;

    let existing_exam_categories= exam_categories_table.getKeyedColumnSampleMap(
        &(exam_categories::pertinent_headers::procedure_code.getLabel())
    )?;

    let mut complete_exam_code_list:Vec<exam_categories::exam_category>=Vec::new();
    
    for procedure_code in main_exam_categories.keys()
    {
        let mut next_member:exam_categories::exam_category=exam_categories::exam_category{
            procedure_code:procedure_code.to_string(),
            exam:"".to_string(),
            subspecialty:"".to_string(),
            comments:"".to_string()
        };

        match existing_exam_categories.get(procedure_code)
        {
            None=>{
                println!("Couldn't find procedure code {}",procedure_code.to_string());
                let sample_row_index = match main_exam_categories.get(procedure_code){
                    Some(x)=>x,
                    None=>{return Err(format!("Coudldn't get sample row {} ",procedure_code));}
                };
                next_member.procedure_code=main_data_table.getVal(&main_headers::pertinent_headers::procedure_code.getLabel(), sample_row_index)?;
                next_member.exam=main_data_table.getVal(&main_headers::pertinent_headers::exam.getLabel(), sample_row_index)?;
            },
            Some(sample_row_index)=>{
                next_member.procedure_code=exam_categories_table.getVal(&exam_categories::pertinent_headers::procedure_code.getLabel(), sample_row_index)?;
                next_member.exam=exam_categories_table.getVal(&exam_categories::pertinent_headers::exam.getLabel(), sample_row_index)?;
                next_member.subspecialty=exam_categories_table.getVal(&exam_categories::pertinent_headers::subspecialty.getLabel(), sample_row_index)?;
                next_member.comments=exam_categories_table.getVal(&exam_categories::pertinent_headers::comments.getLabel(), sample_row_index)?;
            }
        }

        complete_exam_code_list.push(next_member);  
    }

    complete_exam_code_list.sort();

    return Ok(complete_exam_code_list);
}

fn get_locations_list(
    main_data_table:&table::Table,
    exam_locations_table:&table::Table
)->Result<Vec<location_categories::location_category>,String>
{
    let main_exam_locations= main_data_table.getKeyedColumnSampleMap(
        &(main_headers::pertinent_headers::location.getLabel())
    )?;

    let existing_exam_locations= exam_locations_table.getKeyedColumnSampleMap(
        &(location_categories::pertinent_headers::location.getLabel())
    )?;

    let mut complete_exam_location_list:Vec<location_categories::location_category>=Vec::new();
    
    for location in main_exam_locations.keys()
    {
        let mut next_member:location_categories::location_category=location_categories::location_category{
            location:location.to_string(),
            context:"".to_string(),
            comments:"".to_string()
        };

        match existing_exam_locations.get(location)
        {
            None=>{
                println!("Couldn't find location {}",location.to_string());
                let sample_row_index = match main_exam_locations.get(location){
                    Some(x)=>x,
                    None=>{return Err(format!("Coudldn't get sample row {} ",location));}
                };
                next_member.location=main_data_table.getVal(&main_headers::pertinent_headers::location.getLabel(), sample_row_index)?;
            },
            Some(sample_row_index)=>{
                next_member.location=exam_locations_table.getVal(&location_categories::pertinent_headers::location.getLabel(), sample_row_index)?;
                next_member.context=exam_locations_table.getVal(&location_categories::pertinent_headers::context.getLabel(), sample_row_index)?;
                next_member.comments=exam_locations_table.getVal(&location_categories::pertinent_headers::comments.getLabel(), sample_row_index)?;
            }
        }

        complete_exam_location_list.push(next_member);  
    }

    complete_exam_location_list.sort();

    return Ok(complete_exam_location_list);
}

fn backup(dt:DateTime<Local>,p:String,label:String)->Result<u64,std::io::Error>
{
    let backup_path="./categories/archive/".to_string() + &dt.timestamp().to_string() + " backup of " + &label;
    println!("Backup to {}",backup_path);
    return fs::copy(p.to_owned(),backup_path);
}


fn main()->Result<(), Box<dyn Error>> {
    let main_data_table=table::Table::create(file_names::MAIN_DATA_FILE)?;

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

    let map = match rvu_map::createMap(&main_data_table,&exam_to_subspecialty_map,&location_to_context_map)
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
