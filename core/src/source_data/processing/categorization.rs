use std::collections::{HashMap, HashSet};

use crate::{globals::{bvu_headers, file_names::{self, UNACCOUNTED_CATEGORIES_FILE}, main_headers}, source_data::tables::{bvu_map::BVUMap, exam_categories::{ExamCategoryEntry, Exam_Categories}, exam_data::{Exam, ExamTable}, location_categories::{LocationCategoryEntry, Location_Categories}}};

pub(crate) fn check_categories_list(
    main_data_table: ExamTable,
    exam_categories_table: &Exam_Categories,
) -> Result<(), String> {
    let main_exam_categories = main_data_table.get_procedure_codes();
    let existing_exam_categories = exam_categories_table.get_procedure_codes();

    let mut unaccounted_codes: HashSet<String> = HashMap::new();

    for procedure_code in main_exam_categories {
        if !existing_exam_categories.contains(procedure_code.as_str()) {
            unaccounted_codes.insert(procedure_code);
        }
    }

    if unaccounted_codes.len()>0
    {
        let mut errtable = exam_categories_table.structural_clone();
        let mut keys:Vec<&String>=unaccounted_codes.keys().collect();
        keys.sort();
        for key in keys
        {
            let val=unaccounted_codes.get(key).expect("Should be here.");
            errtable.pushrow(
                Vec::from([
                    key.to_string(),
                    val.to_string(),
                    "".to_string(),
                    "".to_string()
                ])
            )
        }
        errtable.write_to_file(UNACCOUNTED_CATEGORIES_FILE.to_string());
        Err("Unaccounted codes.".to_string())
    }
    else {
        let _ = std::fs::remove_file(UNACCOUNTED_CATEGORIES_FILE.to_string());
        Ok(())
    }
}

pub(crate) fn get_site_and_location_context_map(exam_locations_table: &Location_Categories) -> Result<HashMap<u64,HashMap<String,String>>,String>{
    let mut err=false;

    let mut result:HashMap<u64,HashMap<String,String>>=HashMap::new();

    let process_location = |entry:LocationCategoryEntry|->() {

        let sitemap = match result.entry(entry.site_id)
        {
            std::collections::hash_map::Entry::Occupied(x) => x.into_mut(),
            std::collections::hash_map::Entry::Vacant(x) => x.insert(HashMap::new())
        };

        match sitemap.get(&entry.location)
        {
            Some(x) => {
                err=true;
                eprintln!("Duplicate entries in map: {}/{}",entry.site,entry.location)
            },
            None => {sitemap.insert(entry.location,entry.context);}
        };
    };

    exam_locations_table.for_each(process_location);

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

pub fn build_salem_rvumap(main_data_table: &ExamTable) -> Result<HashMap<String, f64>, String> {
    let mut retval: HashMap<String, f64> = HashMap::new();

    let mut rvu_sum: f64 = 0.0;
    let mut rvu_disc: f64 = 0.0;

    for row_i in main_data_table.row_indices() {
        let rawval=main_data_table.get_val(&main_headers::PertinentHeaders::Rvu.get_label(), &row_i)?;
        let rvus = match rawval.parse::<f64>()
        {
            Ok(x) => x,
            Err(e) => {
                match rawval.as_str()
                {
                    "NULL" => 0f64,
                    _ => {return Err(format!("{:?} trying to parse {}", e, rawval));}
                }
            }
        };
        rvu_sum += rvus;

        let exam_code = main_data_table.get_val(
            &main_headers::PertinentHeaders::ProcedureCode.get_label(),
            &row_i,
        )?;

        let current = retval.get(&exam_code);
        match current {
            Some(&x) => {
                if x != rvus {
                    rvu_disc += (x - rvus).abs();
                    if rvus > x {
                        println!("Replacing RVUs for exam code {} with higher value found {}, previously {}",exam_code,rvus,x);
                        retval.insert(exam_code, rvus);
                    }
                }
            }
            None => {
                retval.insert(exam_code, rvus);
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

//Check BVU source for missing exam codes. Also puts all exam descriptions in comments.
pub fn check_bvusource(main_data_table: &ExamTable, bvu_data_table: &BVUMap) {
    let mut found: HashSet<String> = HashSet::new();
    for row_i in main_data_table.row_indices() {
        let exam_code = main_data_table
            .get_val(
                &main_headers::PertinentHeaders::ProcedureCode.get_label(),
                &row_i,
            )
            .expect("Couldn't get exam code from table!");

        if !found.contains(&exam_code) {
            found.insert(exam_code.to_owned());

            let mut bvu_table_row: Option<usize> = None;
            for row_b in bvu_data_table.row_indices() {
                let this_code = bvu_data_table
                    .get_val(&bvu_headers::PertinentHeaders::ExamCode.get_label(), &row_b)
                    .expect("Couldn't get bvu row");
                if this_code == exam_code {
                    bvu_table_row = Some(row_b);
                    break;
                }
            }
            let desc = main_data_table
                .get_val(&main_headers::PertinentHeaders::Exam.get_label(), &row_i)
                .expect("Couldn't get exam description from table!");

            match bvu_table_row {
                Some(bvu_table_row) => {
                    bvu_data_table
                        .set_val(
                            &bvu_headers::PertinentHeaders::ExamDescription.get_label(),
                            &bvu_table_row,
                            &desc,
                        )
                        .expect("Couldn't modify bvu data table.");
                }
                None => {
                    let mut newrow: Vec<String> = Vec::new();
                    newrow.push(exam_code);
                    for _ in 0..6 {
                        newrow.push("".to_string());
                    }
                    newrow.push(desc);
                    newrow.push("".to_string());
                    bvu_data_table.pushrow(newrow);
                }
            }
        }
    }

    bvu_data_table.write_to_file(file_names::BVU_UPDATE_FILE.to_string());
}

pub fn build_salem_bvumap(bvu_data_table: &BVUMap) -> Result<HashMap<String, f64>, String> {
    let mut retval: HashMap<String, f64> = HashMap::new();

    for row_i in bvu_data_table.row_indices() {
        let rawval=bvu_data_table
        .get_val(
            &bvu_headers::PertinentHeaders::TargetPercentile.get_label(),
            &row_i,
        )?;
        let rvus = match rawval
            .parse::<f64>()
        {
            Ok(x) => x,
            Err(e) => {
                return Err(format!("{:?} for {}", e,rawval));
            }
        };

        let exam_code =
            bvu_data_table.get_val(&bvu_headers::PertinentHeaders::ExamCode.get_label(), &row_i)?;

        let current = retval.get(&exam_code);
        match current {
            Some(&_x) => {
                eprintln!("Duplicate BVU table entires for {}", exam_code);
            }
            None => {
                retval.insert(exam_code, rvus);
            }
        };
    }

    Ok(retval)
}

/*
pub fn build_salem_modality_map(
    main_data_table: &Table,
) -> Result<HashMap<String, String>, String> {
    let mut retval: HashMap<String, String> = HashMap::new();

    for row_i in main_data_table.row_indices() {
        let exam_code = main_data_table.get_val(
            &main_headers::PertinentHeaders::ProcedureCode.get_label(),
            &row_i,
        )?;

        if let std::collections::hash_map::Entry::Vacant(e) = retval.entry(exam_code) {
            let listed_modality = main_data_table.get_val(
                &main_headers::PertinentHeaders::Modality.get_label(),
                &row_i,
            )?;
            e.insert(listed_modality);
        }
    }

    Ok(retval)
}
*/
