use std::{
    collections::{HashMap, HashSet},
    fs,
};

use chrono::{DateTime, Local};

use crate::{
    globals::{bvu_headers, file_names, main_headers},
    table,
};

pub mod exam_categories {
    use std::cmp::Ordering;

    use serde::{Deserialize, Serialize};

    pub(crate) enum PertinentHeaders {
        procedure_code,
        exam,
        subspecialty,
        comments,
    }

    impl PertinentHeaders {
        pub(crate) fn get_label(&self) -> String {
            match self {
                PertinentHeaders::procedure_code => "Exam Code".to_string(),
                PertinentHeaders::exam => "Exam Description".to_string(),
                PertinentHeaders::subspecialty => "Subspecialty".to_string(),
                PertinentHeaders::comments => "Comments".to_string(),
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct exam_category {
        pub procedure_code: String,
        pub exam: String,
        pub subspecialty: String,
        pub comments: String,
    }

    impl Eq for exam_category {}

    impl PartialOrd for exam_category {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for exam_category {
        fn eq(&self, other: &Self) -> bool {
            self.procedure_code == other.procedure_code && self.exam == other.exam
            //self.subspecialty == other.subspecialty &&
            //self.comments == other.comments
        }
    }

    impl Ord for exam_category {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.exam.cmp(&other.exam) {
                std::cmp::Ordering::Equal => self.procedure_code.cmp(&other.procedure_code),
                examcmp => examcmp,
            }
        }
    }
}

pub(crate) mod location_categories {
    use std::cmp::Ordering;

    pub(crate) enum pertinent_headers {
        location,
        context,
        comments,
    }

    impl pertinent_headers {
        pub(crate) fn getLabel(&self) -> String {
            match self {
                pertinent_headers::location => "Location".to_string(),
                pertinent_headers::context => "Context".to_string(),
                pertinent_headers::comments => "Comments".to_string(),
            }
        }
    }

    pub(crate) struct location_category {
        pub location: String,
        pub context: String,
        pub comments: String,
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

pub(crate) fn get_categories_list(
    main_data_table: &table::Table,
    exam_categories_table: &table::Table,
) -> Result<Vec<exam_categories::exam_category>, String> {
    let main_exam_categories = main_data_table
        .getKeyedColumnSampleMap(&(main_headers::pertinent_headers::procedure_code.getLabel()))?;

    let existing_exam_categories = exam_categories_table.getKeyedColumnSampleMap(
        &(exam_categories::PertinentHeaders::procedure_code.get_label()),
    )?;

    let mut complete_exam_code_list: Vec<exam_categories::exam_category> = Vec::new();

    for procedure_code in main_exam_categories.keys() {
        let mut next_member: exam_categories::exam_category = exam_categories::exam_category {
            procedure_code: procedure_code.to_string(),
            exam: "".to_string(),
            subspecialty: "".to_string(),
            comments: "".to_string(),
        };

        match existing_exam_categories.get(procedure_code) {
            None => {
                println!(
                    "Couldn't find procedure code {}",
                    procedure_code
                );
                let sample_row_index = match main_exam_categories.get(procedure_code) {
                    Some(x) => x,
                    None => {
                        return Err(format!("Coudldn't get sample row {} ", procedure_code));
                    }
                };
                next_member.procedure_code = main_data_table.getVal(
                    &main_headers::pertinent_headers::procedure_code.getLabel(),
                    sample_row_index,
                )?;
                next_member.exam = main_data_table.getVal(
                    &main_headers::pertinent_headers::exam.getLabel(),
                    sample_row_index,
                )?;
            }
            Some(sample_row_index) => {
                next_member.procedure_code = exam_categories_table.getVal(
                    &exam_categories::PertinentHeaders::procedure_code.get_label(),
                    sample_row_index,
                )?;
                next_member.exam = exam_categories_table.getVal(
                    &exam_categories::PertinentHeaders::exam.get_label(),
                    sample_row_index,
                )?;
                next_member.subspecialty = exam_categories_table.getVal(
                    &exam_categories::PertinentHeaders::subspecialty.get_label(),
                    sample_row_index,
                )?;
                next_member.comments = exam_categories_table.getVal(
                    &exam_categories::PertinentHeaders::comments.get_label(),
                    sample_row_index,
                )?;
            }
        }

        complete_exam_code_list.push(next_member);
    }

    complete_exam_code_list.sort();

    Ok(complete_exam_code_list)
}

pub(crate) fn get_locations_list(
    main_data_table: &table::Table,
    exam_locations_table: &table::Table,
) -> Result<Vec<location_categories::location_category>, String> {
    let main_exam_locations = main_data_table
        .getKeyedColumnSampleMap(&(main_headers::pertinent_headers::location.getLabel()))?;

    let existing_exam_locations = exam_locations_table
        .getKeyedColumnSampleMap(&(location_categories::pertinent_headers::location.getLabel()))?;

    let mut complete_exam_location_list: Vec<location_categories::location_category> = Vec::new();

    for location in main_exam_locations.keys() {
        let mut next_member: location_categories::location_category =
            location_categories::location_category {
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
                next_member.location = main_data_table.getVal(
                    &main_headers::pertinent_headers::location.getLabel(),
                    sample_row_index,
                )?;
            }
            Some(sample_row_index) => {
                next_member.location = exam_locations_table.getVal(
                    &location_categories::pertinent_headers::location.getLabel(),
                    sample_row_index,
                )?;
                next_member.context = exam_locations_table.getVal(
                    &location_categories::pertinent_headers::context.getLabel(),
                    sample_row_index,
                )?;
                next_member.comments = exam_locations_table.getVal(
                    &location_categories::pertinent_headers::comments.getLabel(),
                    sample_row_index,
                )?;
            }
        }

        complete_exam_location_list.push(next_member);
    }

    complete_exam_location_list.sort();

    Ok(complete_exam_location_list)
}

pub fn backup(dt: DateTime<Local>, p: String, label: String) -> Result<u64, std::io::Error> {
    let backup_path =
        "./categories/archive/".to_string() + &dt.timestamp().to_string() + " backup of " + &label;
    println!("Backup to {}", backup_path);
    fs::copy(p, backup_path)
}

pub fn buildSalemRVUMap(main_data_table: &table::Table) -> Result<HashMap<String, f64>, String> {
    let mut retval: HashMap<String, f64> = HashMap::new();

    let mut rvu_sum: f64 = 0.0;
    let mut rvu_disc: f64 = 0.0;

    for row_i in main_data_table.rowIndices() {
        let rvus = match main_data_table
            .getVal(&main_headers::pertinent_headers::rvu.getLabel(), &row_i)?
            .parse::<f64>()
        {
            Ok(x) => x,
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        };
        rvu_sum += rvus;

        let exam_code = main_data_table.getVal(
            &main_headers::pertinent_headers::procedure_code.getLabel(),
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
pub fn checkBVUSource(main_data_table: &table::Table, bvu_data_table: &mut table::Table) {
    let mut found: HashSet<String> = HashSet::new();
    for row_i in main_data_table.rowIndices() {
        let exam_code = main_data_table
            .getVal(
                &main_headers::pertinent_headers::procedure_code.getLabel(),
                &row_i,
            )
            .expect("Couldn't get exam code from table!");

        if !found.contains(&exam_code) {
            found.insert(exam_code.to_owned());

            let mut bvu_table_row: Option<usize> = None;
            for row_b in bvu_data_table.rowIndices() {
                let this_code = bvu_data_table
                    .getVal(
                        &bvu_headers::pertinent_headers::exam_code.getLabel(),
                        &row_b,
                    )
                    .expect("Couldn't get bvu row");
                if this_code == exam_code {
                    bvu_table_row = Some(row_b);
                    break;
                }
            }
            let desc = main_data_table
                .getVal(&main_headers::pertinent_headers::exam.getLabel(), &row_i)
                .expect("Couldn't get exam description from table!");

            match bvu_table_row {
                Some(bvu_table_row) => {
                    bvu_data_table
                        .setVal(
                            &bvu_headers::pertinent_headers::exam_description.getLabel(),
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

pub fn buildSalemBVUMap(bvu_data_table: &table::Table) -> Result<HashMap<String, f64>, String> {
    let mut retval: HashMap<String, f64> = HashMap::new();

    for row_i in bvu_data_table.rowIndices() {
        let rvus = match bvu_data_table
            .getVal(
                &bvu_headers::pertinent_headers::target_percentile.getLabel(),
                &row_i,
            )?
            .parse::<f64>()
        {
            Ok(x) => x,
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        };

        let exam_code = bvu_data_table.getVal(
            &bvu_headers::pertinent_headers::exam_code.getLabel(),
            &row_i,
        )?;

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

pub fn buildSalemModalityMap(
    main_data_table: &table::Table,
) -> Result<HashMap<String, String>, String> {
    let mut retval: HashMap<String, String> = HashMap::new();

    for row_i in main_data_table.rowIndices() {
        let exam_code = main_data_table.getVal(
            &main_headers::pertinent_headers::procedure_code.getLabel(),
            &row_i,
        )?;

        if let std::collections::hash_map::Entry::Vacant(e) = retval.entry(exam_code) {
            let listed_modality = main_data_table.getVal(
                &main_headers::pertinent_headers::modality.getLabel(),
                &row_i,
            )?;
            e.insert(listed_modality);
        }
    }

    Ok(retval)
}
