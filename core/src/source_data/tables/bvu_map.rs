use std::{collections::{HashMap, HashSet}, error::Error};

use chrono::{NaiveDateTime};
use serde::Deserialize;

use super::table::Table;

pub struct BVUMapEntry {
    pub exam_code:String,
    pub bvu:String,
}

const EXAM_CODE_HEADER:&str="Location group";
const BVU_HEADER:&str="50th";

pub struct BVUMap {
    filename:String
}

impl Table<BVUMapEntry> for BVUMap
{
    fn get_file_path(&self)->&str {&self.filename}

    fn build_from_headers_and_row(header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<BVUMapEntry, Box<dyn std::error::Error>>{
        Ok(
            BVUMapEntry{
                exam_code:Self::get_from_row_with_header(EXAM_CODE_HEADER, header_map, row),
                bvu: Self::get_from_row_with_header(BVU_HEADER, header_map, row),
            }
        )
    }
}

impl BVUMap {
    pub fn create(filename:&str)->BVUMap{BVUMap{filename:filename.to_string()}}
}