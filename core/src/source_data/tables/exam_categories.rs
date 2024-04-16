use std::{collections::{HashMap, HashSet}, error::Error};

use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::serialization::output::JSONFileOut;

use super::{table::Table, types::{ExamCode, Subspecialty}};

pub struct ExamCategoryEntry {
    pub exam_code:ExamCode,
    pub subspecialty:Subspecialty,
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
            subspecialty: self.subspecialty.to_string()
        }
    }
}

pub const EXAM_CODE_HEADER:&str="Exam Code";
pub const SUBSPECIALTY_HEADER:&str="Subspecialty";

pub struct Exam_Categories {
    filename:String
}

impl Table<ExamCategoryEntry> for Exam_Categories
{
    fn get_file_path(&self)->&str {&self.filename}

    fn build_from_headers_and_row(header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<ExamCategoryEntry, Box<dyn std::error::Error>>{
        Ok(
            ExamCategoryEntry{
                exam_code:Self::get_from_row_with_header(EXAM_CODE_HEADER, header_map, row),
                subspecialty: Self::get_from_row_with_header(SUBSPECIALTY_HEADER, header_map, row),
            }
        )
    }
}

impl Exam_Categories {
    pub fn create(filename:&str)->Exam_Categories{Exam_Categories{filename:filename.to_string()}}
    pub fn get_procedure_codes(&self)->HashSet<String>{
        let mut retval:HashSet<String>=HashSet::new();
        for entry in self.iter()
        {
            if !retval.insert(entry.exam_code.to_string()){                
                eprintln!("Procedure code {} is duplicated in {}",&entry.exam_code,self.filename);
            }
        }
        retval
    }
}