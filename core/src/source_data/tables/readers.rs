use std::{collections::{HashMap, HashSet}, error::Error};

use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

use super::{table::Table, types::{ExamCode, ExamDescription, Location}};

#[derive(Debug,Serialize,Deserialize,Ord,PartialOrd,Eq,PartialEq)]
pub struct ExamReader {
    pub signer_acct_id:u64, //SignerAcctID
    pub rad_last_name:String, //RadLastNm
    pub rad_first_name:String, //RadFirstNm
    pub excluded:bool, //Excluded
}

const SIGNER_ACCT_ID_HEADER:&str="SignerAcctID";
const RAD_LAST_NAME_HEADER:&str="RadLastNm";
const RAD_FIRST_NAME_HEADER:&str="RadFirstNm";
const EXCLUDED:&str="Excluded";

pub struct ReaderTable {
    filename:String
}

impl Table<ExamReader> for ReaderTable
{
    fn get_file_path(&self)->&str {&self.filename}

    fn build_from_headers_and_row(header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<ExamReader, Box<dyn std::error::Error>>{

        Ok(
            ExamReader{               
                signer_acct_id: Self::parse(SIGNER_ACCT_ID_HEADER,header_map,row)?,
                rad_last_name: Self::get_from_row_with_header(RAD_LAST_NAME_HEADER, header_map, row),
                rad_first_name: Self::get_from_row_with_header(RAD_FIRST_NAME_HEADER, header_map, row),
                excluded: Self::parse(EXCLUDED,header_map,row)?
            }
        )
    }
}

impl ReaderTable
{
    pub fn create(filename:&str)->ReaderTable{ReaderTable{filename:filename.to_string()}}
    pub fn headers()->[String;4]
    {
        [
            SIGNER_ACCT_ID_HEADER.to_string(),
            RAD_FIRST_NAME_HEADER.to_string(),
            RAD_FIRST_NAME_HEADER.to_string(),
            EXCLUDED.to_string()
        ]
    }
}