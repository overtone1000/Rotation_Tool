use std::{collections::{BTreeMap, HashMap, HashSet}, f32::consts::E, io::ErrorKind};

use crate::{globals::{bvu_headers, file_names::{self, UNACCOUNTED_EXAM_CODES_FILE}, main_headers}, source_data::tables::{bvu_map::{BVUMap, BVUMapEntry}, exam_categories::{ExamCategoryEntry, Exam_Categories, EXAM_CODE_HEADER, SUBSPECIALTY_HEADER}, exam_data::{Exam, ExamTable}, location_categories::{LocationCategoryEntry, Location_Categories}, table::Table, types::{Context, Location}}};

pub(crate) fn check_categories_list(
    main_data: &Vec<Exam>,
    exam_categories_table: &Exam_Categories,
) -> Result<(), Box<dyn std::error::Error>> {
    let existing_exam_categories = exam_categories_table.get_procedure_codes();

    let mut unaccounted_codes: HashMap<String,String> = HashMap::new();

    for exam in main_data {
        if !existing_exam_categories.contains(exam.exam_code.as_str()) {
            unaccounted_codes.insert(exam.exam_code.to_owned(),exam.procedure_description.to_owned());
        }
    }

    if unaccounted_codes.len()>0
    {
        let headers=[
            crate::source_data::tables::exam_categories::EXAM_CODE_HEADER.to_owned(),
            crate::source_data::tables::exam_categories::SUBSPECIALTY_HEADER.to_owned()];
        let mut entries:Vec<Vec<String>>=Vec::new();
        for (code,desc) in unaccounted_codes
        {
            entries.push(vec![
                code,
                desc
            ])
        }
        
        ExamTable::write(UNACCOUNTED_EXAM_CODES_FILE,&headers,entries)?;
        Err(Box::new(std::io::Error::new(ErrorKind::InvalidData,"Unaccounted exam codes.".to_string())))
    }
    else {
        let _ = std::fs::remove_file(UNACCOUNTED_EXAM_CODES_FILE.to_string());
        Ok(())
    }
}

pub(crate) fn get_site_and_location_context_map(exam_locations_table: &Location_Categories) -> Result<BTreeMap<u64,BTreeMap<Location,Context>>,String>{
    let mut err=false;

    let mut result:BTreeMap<u64,BTreeMap<String,String>>=BTreeMap::new();

    for entry in exam_locations_table.iter()
    {
        let sitemap = match result.entry(entry.site_id)
        {
            std::collections::btree_map::Entry::Occupied(x) => x.into_mut(),
            std::collections::btree_map::Entry::Vacant(x) => x.insert(BTreeMap::new())
        };

        match sitemap.get(&entry.location)
        {
            Some(x) => {
                err=true;
                eprintln!("Duplicate entries in map: {}/{}",entry.site_id,entry.location)
            },
            None => {
                sitemap.insert(entry.location,entry.context);
            }
        };
    }

    //println!("Context map:{:?}",result);

    if err {return Err("Error building exam context map.".to_string());}
    Ok(result)
}

/*
pub(crate) fn get_locations_list(
    main_data_table: &Table,
    exam_locations_table: &Table,
) -> Result<Vec<location_categories::LocationCategory>, String> {
    
    //TODO
    //Added "for_each" function to table and modified "Categories_Location.csv" to include site IDs as may not be able to rely strictly on location.
    //Added site ID to the LocationCategory, thus the error below. Need to test against site ID and location.

    let main_exam_locations = main_data_table
        .get_keyed_column_sample_map(&(main_headers::PertinentHeaders::Location.get_label()))?;

    let existing_exam_locations = exam_locations_table.get_keyed_column_sample_map(
        &(location_categories::PertinentHeaders::Location.get_label()),
    )?;

    let mut complete_exam_location_list: Vec<location_categories::LocationCategory> = Vec::new();

    for location in main_exam_locations.keys() {
        let mut next_member: location_categories::LocationCategory =
            location_categories::LocationCategory {
                location: location.to_string(),
                context: "".to_string(),
                comments: "".to_string(),
            };

        match existing_exam_locations.get(location) {
            None => {
                println!("Couldn't find location {}", location);
                let sample_row_index = match main_exam_locations.get(location) {
                    Some(x) => x,
                    None => {
                        return Err(format!("Coudldn't get sample row {} ", location));
                    }
                };
                next_member.location = main_data_table.get_val(
                    &main_headers::PertinentHeaders::Location.get_label(),
                    sample_row_index,
                )?;
            }
            Some(sample_row_index) => {
                next_member.location = exam_locations_table.get_val(
                    &location_categories::PertinentHeaders::Location.get_label(),
                    sample_row_index,
                )?;
                next_member.context = exam_locations_table.get_val(
                    &location_categories::PertinentHeaders::Context.get_label(),
                    sample_row_index,
                )?;
                next_member.comments = exam_locations_table.get_val(
                    &location_categories::PertinentHeaders::Comments.get_label(),
                    sample_row_index,
                )?;
            }
        }

        complete_exam_location_list.push(next_member);
    }

    complete_exam_location_list.sort();

    Ok(complete_exam_location_list)
}
*/

pub fn build_salem_rvumap(main_data_table: &Vec<Exam>) -> Result<HashMap<String, f64>, String> {
    let mut retval: HashMap<String, f64> = HashMap::new();

    let mut rvu_sum: f64 = 0.0;
    let mut rvu_disc: f64 = 0.0;

    for entry in main_data_table.iter()
    {
        rvu_sum += entry.rvu;

        let current = retval.get(&entry.exam_code);
        match current {
            Some(&x) => {
                if x != entry.rvu {
                    rvu_disc += (x - entry.rvu).abs();
                    if entry.rvu > x {
                        println!("Replacing RVUs for exam code {} with higher value found {}, previously {}",entry.exam_code,entry.rvu,x);
                        retval.insert(entry.exam_code.to_owned(), entry.rvu);
                    }
                }
            }
            None => {
                retval.insert(entry.exam_code.to_owned(), entry.rvu);
            }
        };
    }

    let percentage = rvu_disc / rvu_sum * 100.0;
    println!(
        "RVU discrepancy is {} of {} or {}% of the data set.",
        rvu_disc, rvu_sum, percentage
    );

    Ok(retval)
}

//Check BVU source for missing exam codes.
pub fn check_bvusource(main_data: &Vec<Exam>, bvu_data_table: &BVUMap) -> Result<(), Box<dyn std::error::Error>>{
    let mut bvu_exam_codes:HashSet<String>=HashSet::new();
    for bvu_entry in bvu_data_table.iter()
    {
        bvu_exam_codes.insert(bvu_entry.exam_code);
    }

    let mut missing_codes:HashMap<String,String>=HashMap::new();
    for main_data_table_entry in main_data
    {
        if !bvu_exam_codes.contains(&main_data_table_entry.exam_code)
        {
            missing_codes.insert(main_data_table_entry.exam_code.to_owned(), main_data_table_entry.procedure_description.to_owned());
        }
    }
    
    if missing_codes.len()>0
    {
        let mut missing_codes_vec:Vec<&String>=missing_codes.keys().collect();
        missing_codes_vec.sort();

        let mut vecofvec:Vec<Vec<String>>=Vec::new();
        for missing_code in missing_codes_vec
        {
            vecofvec.push(vec!(missing_code.to_string(),missing_codes.get(missing_code).expect("Should have value").to_string()));
        }
        BVUMap::write(file_names::BVU_UPDATE_FILE,&[EXAM_CODE_HEADER.to_string()],vecofvec).unwrap();

        Err(Box::new(std::io::Error::new(ErrorKind::InvalidData,"Missing BVU codes.")))
    }
    else {
        let _ = std::fs::remove_file(file_names::BVU_UPDATE_FILE.to_string());
        Ok(())
    }
}