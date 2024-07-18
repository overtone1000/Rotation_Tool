use std::collections::{HashMap, HashSet};

use serde::Serialize;

use crate::serialization::output::JSONFileOut;

use super::{
    exam_data::Exam,
    table::{SerializableTable, Table},
    types::{ExamCode, Subspecialty},
};

#[derive(Serialize)]
pub struct ExamCategoryEntry {
    pub exam_code: ExamCode,
    pub exam_description: String,
    pub subspecialty: Subspecialty,
    pub comments: String,
}

impl Eq for ExamCategoryEntry {}

impl PartialOrd for ExamCategoryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ExamCategoryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.exam_code == other.exam_code
    }
}

impl Ord for ExamCategoryEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.exam_code.cmp(&other.exam_code)
    }
}

impl ExamCategoryEntry {
    pub fn copy(&self) -> ExamCategoryEntry {
        ExamCategoryEntry {
            exam_code: self.exam_code.to_string(),
            exam_description: self.exam_description.to_string(),
            subspecialty: self.subspecialty.to_string(),
            comments: self.comments.to_string(),
        }
    }
}

pub const EXAM_CODE_HEADER: &str = "Exam Code";
pub const EXAM_DESCRIPTION_HEADER: &str = "Exam Description";
pub const SUBSPECIALTY_HEADER: &str = "Subspecialty";
pub const COMMENTS_HEADER: &str = "Comments";

pub struct ExamCategories {
    filename: String,
}

impl Table for ExamCategories {
    type Entry = ExamCategoryEntry;
    fn get_file_path(&self) -> &str {
        &self.filename
    }

    fn build_from_headers_and_row(
        header_map: &HashMap<String, usize>,
        row: &Vec<String>,
    ) -> Result<ExamCategoryEntry, Box<dyn std::error::Error>> {
        Ok(ExamCategoryEntry {
            exam_code: Self::get_from_row_with_header(EXAM_CODE_HEADER, header_map, row),
            exam_description: Self::get_from_row_with_header(
                EXAM_DESCRIPTION_HEADER,
                header_map,
                row,
            ),
            subspecialty: Self::get_from_row_with_header(SUBSPECIALTY_HEADER, header_map, row),
            comments: Self::get_from_row_with_header(COMMENTS_HEADER, header_map, row),
        })
    }
}

impl ExamCategories {
    pub fn create(filename: &str) -> ExamCategories {
        ExamCategories {
            filename: filename.to_string(),
        }
    }
    pub fn get_procedure_codes(&self) -> HashSet<String> {
        let mut retval: HashSet<String> = HashSet::new();
        for entry in self.iter() {
            if !retval.insert(entry.exam_code.to_string()) {
                eprintln!(
                    "Procedure code {} is duplicated in {}",
                    &entry.exam_code, self.filename
                );
            }
        }
        retval
    }
}

impl Serialize for ExamCategories {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.table_serialize(serializer)
    }
}
impl JSONFileOut for ExamCategories {}
