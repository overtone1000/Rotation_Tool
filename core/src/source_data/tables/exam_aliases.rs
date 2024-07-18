use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use chrono::NaiveDateTime;
use serde::Deserialize;

use super::{table::Table, types::ExamCode};

pub struct ExamAlias {
    pub alias: ExamCode,
    pub exam_code: ExamCode,
}

const EXAM_CODE_HEADER: &str = "Exam Code";
const ALIAS_HEADER: &str = "Alias";

pub struct Exam_Aliases {
    filename: String,
}

impl Table for Exam_Aliases {
    type Entry = ExamAlias;
    fn get_file_path(&self) -> &str {
        &self.filename
    }

    fn build_from_headers_and_row(
        header_map: &HashMap<String, usize>,
        row: &Vec<String>,
    ) -> Result<ExamAlias, Box<dyn std::error::Error>> {
        Ok(ExamAlias {
            exam_code: Self::get_from_row_with_header(EXAM_CODE_HEADER, header_map, row),
            alias: Self::get_from_row_with_header(ALIAS_HEADER, header_map, row),
        })
    }
}

impl Exam_Aliases {
    pub fn create(filename: &str) -> Exam_Aliases {
        Exam_Aliases {
            filename: filename.to_string(),
        }
    }
}
