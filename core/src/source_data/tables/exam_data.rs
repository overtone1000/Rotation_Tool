use std::{collections::{HashMap, HashSet}, error::Error};

use chrono::{NaiveDateTime};
use serde::Deserialize;

use super::{table::Table, types::{ExamCode, ExamDescription, Location}};

#[derive(Debug)]
pub struct Exam {
    pub accession:String, //Accession
    pub exam_code:ExamCode, //ProcedureCodeList
    pub procedure_description:ExamDescription, //ProcedureDescList
    pub signer_acct_id:u64, //SignerAcctID
    pub rad_last_name:String, //RadLastNm
    pub rad_first_name:String, //RadFirstNm
    pub list_datetime:NaiveDateTime, //ScheduledDatetime
    pub rvu:f64, //WorkRVU
    pub site_id:u64, //SiteID
    pub location:Location,
}

const ACCESSION_HEADER:&str="Accession";
const PROCEDURE_CODE_HEADER:&str="ProcedureCodeList";
const PROCEDURE_DESCRIPTION_HEADER:&str="ProcedureDescList";
const SIGNER_ACCT_ID_HEADER:&str="SignerAcctID";
const RAD_LAST_NAME_HEADER:&str="RadLastNm";
const RAD_FIRST_NAME_HEADER:&str="RadFirstNm";
const LIST_TIME_HEADER:&str="ScheduledDatetime";
const RVU_HEADER:&str="WorkRVU";
const SITE_ID_HEADER:&str="SiteID";
const LOCATION_HEADER:&str="LocationDescription";

pub struct ExamTable {
    filename:String
}

impl Table<Exam> for ExamTable
{
    fn get_file_path(&self)->&str {&self.filename}

    fn build_from_headers_and_row(header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<Exam, Box<dyn std::error::Error>>{

        let list_time_string=Self::get_from_row_with_header(LIST_TIME_HEADER, header_map, row);
        let datetime = NaiveDateTime::parse_from_str(&list_time_string, "%m/%d/%y %H:%M")?;

        Ok(
            Exam{
                accession:Self::get_from_row_with_header(ACCESSION_HEADER, header_map, row),
                exam_code: Self::get_from_row_with_header(PROCEDURE_CODE_HEADER, header_map, row),
                procedure_description: Self::get_from_row_with_header(PROCEDURE_DESCRIPTION_HEADER, header_map, row),
                signer_acct_id: Self::get_from_row_with_header(SIGNER_ACCT_ID_HEADER, header_map, row).parse().expect("Should parse to integer."),
                rad_last_name: Self::get_from_row_with_header(RAD_LAST_NAME_HEADER, header_map, row),
                rad_first_name: Self::get_from_row_with_header(RAD_FIRST_NAME_HEADER, header_map, row),
                list_datetime: datetime,
                rvu: Self::get_from_row_with_header(RVU_HEADER, header_map, row).parse().expect("Should parse to float."),
                site_id: Self::get_from_row_with_header(SITE_ID_HEADER, header_map, row).parse().expect("Should parse to integer."),
                location: Self::get_from_row_with_header(LOCATION_HEADER, header_map, row),
            }
        )
    }
}

impl ExamTable {
    pub fn create(filename:&str)->ExamTable{ExamTable{filename:filename.to_string()}}
    pub fn get_procedure_codes(&self)->HashSet<String>{
        let mut retval:HashSet<String>=HashSet::new();
        for entry in self.iter()
        {
            if !retval.insert(entry.exam_code.to_string()){                
                eprintln!("Procedure code {} is duplicated in {}",entry.exam_code,self.filename);
            }
        }
        retval
    }
}