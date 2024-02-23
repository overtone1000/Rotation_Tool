use std::collections::{HashMap, HashSet};

use crate::globals::{bvu_headers, file_names, main_headers};

use super::{processed_source::ProcessedSource, table::Table};

pub mod exam_categories {
    use std::cmp::Ordering;

    use serde::{Deserialize, Serialize};

    use crate::serialization::output::JSONFileOut;

    pub(crate) enum PertinentHeaders {
        ProcedureCode,
        Exam,
        Subspecialty,
        Comments,
    }

    impl PertinentHeaders {
        pub(crate) fn get_label(&self) -> String {
            match self {
                PertinentHeaders::ProcedureCode => "Exam Code".to_string(),
                PertinentHeaders::Exam => "Exam Description".to_string(),
                PertinentHeaders::Subspecialty => "Subspecialty".to_string(),
                PertinentHeaders::Comments => "Comments".to_string(),
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct ExamCategory {
        pub procedure_code: String,
        pub exam: String,
        pub subspecialty: String,
        pub comments: String,
    }

    impl JSONFileOut for Vec<ExamCategory> {}

    impl Eq for ExamCategory {}

    impl PartialOrd for ExamCategory {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for ExamCategory {
        fn eq(&self, other: &Self) -> bool {
            self.procedure_code == other.procedure_code && self.exam == other.exam
            //self.subspecialty == other.subspecialty &&
            //self.comments == other.comments
        }
    }

    impl Ord for ExamCategory {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.exam.cmp(&other.exam) {
                std::cmp::Ordering::Equal => self.procedure_code.cmp(&other.procedure_code),
                examcmp => examcmp,
            }
        }
    }

    impl ExamCategory {
        pub fn copy(&self) -> ExamCategory {
            ExamCategory {
                procedure_code: self.procedure_code.to_string(),
                exam: self.exam.to_string(),
                subspecialty: self.subspecialty.to_string(),
                comments: self.comments.to_string(),
            }
        }
    }
}

pub(crate) mod location_categories {
    use std::cmp::Ordering;

    pub(crate) enum PertinentHeaders {
        Location,
        Context,
        Comments,
    }

    impl PertinentHeaders {
        pub(crate) fn get_label(&self) -> String {
            match self {
                PertinentHeaders::Location => "Location".to_string(),
                PertinentHeaders::Context => "Context".to_string(),
                PertinentHeaders::Comments => "Comments".to_string(),
            }
        }
    }

    pub(crate) struct LocationCategory {
        pub location: String,
        pub context: String,
        pub comments: String,
    }

    impl Eq for LocationCategory {}

    impl PartialOrd for LocationCategory {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for LocationCategory {
        fn eq(&self, other: &Self) -> bool {
            self.location == other.location
            //self.context == other.context &&
            //self.comments == other.comments
        }
    }

    impl Ord for LocationCategory {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.location.cmp(&other.location)
        }
    }
}

pub(crate) fn get_categories_list(
    main_data_table: &Table,
    exam_categories_table: &Table,
) -> Result<Vec<exam_categories::ExamCategory>, String> {
    let main_exam_categories = main_data_table.get_keyed_column_sample_map(
        &(main_headers::PertinentHeaders::ProcedureCode.get_label()),
    )?;

    let existing_exam_categories = exam_categories_table.get_keyed_column_sample_map(
        &(exam_categories::PertinentHeaders::ProcedureCode.get_label()),
    )?;

    let mut complete_exam_code_list: Vec<exam_categories::ExamCategory> = Vec::new();

    for procedure_code in main_exam_categories.keys() {
        let mut next_member: exam_categories::ExamCategory = exam_categories::ExamCategory {
            procedure_code: procedure_code.to_string(),
            exam: "".to_string(),
            subspecialty: "".to_string(),
            comments: "".to_string(),
        };

        match existing_exam_categories.get(procedure_code) {
            None => {
                println!("Couldn't find procedure code {}", procedure_code);
                let sample_row_index = match main_exam_categories.get(procedure_code) {
                    Some(x) => x,
                    None => {
                        return Err(format!("Coudldn't get sample row {} ", procedure_code));
                    }
                };
                next_member.procedure_code = main_data_table.get_val(
                    &main_headers::PertinentHeaders::ProcedureCode.get_label(),
                    sample_row_index,
                )?;
                next_member.exam = main_data_table.get_val(
                    &main_headers::PertinentHeaders::Exam.get_label(),
                    sample_row_index,
                )?;
            }
            Some(sample_row_index) => {
                next_member.procedure_code = exam_categories_table.get_val(
                    &exam_categories::PertinentHeaders::ProcedureCode.get_label(),
                    sample_row_index,
                )?;
                next_member.exam = exam_categories_table.get_val(
                    &exam_categories::PertinentHeaders::Exam.get_label(),
                    sample_row_index,
                )?;
                next_member.subspecialty = exam_categories_table.get_val(
                    &exam_categories::PertinentHeaders::Subspecialty.get_label(),
                    sample_row_index,
                )?;
                next_member.comments = exam_categories_table.get_val(
                    &exam_categories::PertinentHeaders::Comments.get_label(),
                    sample_row_index,
                )?;
            }
        }

        complete_exam_code_list.push(next_member);
    }

    complete_exam_code_list.sort();

    Ok(complete_exam_code_list)
}

pub(crate) fn get_categories_map(
    source: &ProcessedSource,
) -> Result<HashMap<String, exam_categories::ExamCategory>, String> {
    let list = get_categories_list(&source.main_data_table, &source.exam_categories_table)?;
    let mut retval: HashMap<String, exam_categories::ExamCategory> = HashMap::new();

    for member in &list {
        match retval.entry(member.procedure_code.to_string()) {
            std::collections::hash_map::Entry::Occupied(_) => panic!("Duplicate procedure code!"),
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(member.copy());
            }
        }
    }

    Ok(retval)
}

pub(crate) fn get_locations_list(
    main_data_table: &Table,
    exam_locations_table: &Table,
) -> Result<Vec<location_categories::LocationCategory>, String> {
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

pub fn build_salem_rvumap(main_data_table: &Table) -> Result<HashMap<String, f64>, String> {
    let mut retval: HashMap<String, f64> = HashMap::new();

    let mut rvu_sum: f64 = 0.0;
    let mut rvu_disc: f64 = 0.0;

    for row_i in main_data_table.row_indices() {
        let rvus = match main_data_table
            .get_val(&main_headers::PertinentHeaders::Rvu.get_label(), &row_i)?
            .parse::<f64>()
        {
            Ok(x) => x,
            Err(e) => {
                return Err(format!("{:?}", e));
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
pub fn check_bvusource(main_data_table: &Table, bvu_data_table: &mut Table) {
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

pub fn build_salem_bvumap(bvu_data_table: &Table) -> Result<HashMap<String, f64>, String> {
    let mut retval: HashMap<String, f64> = HashMap::new();

    for row_i in bvu_data_table.row_indices() {
        let rvus = match bvu_data_table
            .get_val(
                &bvu_headers::PertinentHeaders::TargetPercentile.get_label(),
                &row_i,
            )?
            .parse::<f64>()
        {
            Ok(x) => x,
            Err(e) => {
                return Err(format!("{:?}", e));
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
