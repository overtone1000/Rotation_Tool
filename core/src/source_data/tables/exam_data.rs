use std::{collections::HashMap, error::Error};

use chrono::{NaiveDateTime};
use serde::Deserialize;

use super::table::Table;

pub struct Exam {
    pub accession:String, //Accession
    pub procedure_code:String, //ProcedureCodeList
    pub procedure_description:String, //ProcedureDescList
    pub signer_acct_id:u64, //SignerAcctID
    pub rad_last_name:String, //RadLastNm
    pub rad_first_name:String, //RadFirstNm
    pub list_time:NaiveDateTime, //ScheduledDatetime
    pub rvu:f64, //WorkRVU
    pub site_id:u64, //SiteID
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

pub struct ExamTable {
    
}

impl Table<Exam> for ExamTable
{
    fn build_from_headers_and_row(header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<Exam,std::io::Error>{

        let list_time_string=Self::get_from_row_with_header(LIST_TIME_HEADER, header_map, row);
        let datetime = match NaiveDateTime::parse_from_str(&list_time_string, "%m/%d/%y %H:%M") {
            Ok(x) => x,
            Err(x) => {
                return Err(std::io::Error::other(x));
            }
        };

        Ok(
            Exam{
                accession:Self::get_from_row_with_header(ACCESSION_HEADER, header_map, row),
                procedure_code: Self::get_from_row_with_header(PROCEDURE_CODE_HEADER, header_map, row),
                procedure_description: Self::get_from_row_with_header(PROCEDURE_DESCRIPTION_HEADER, header_map, row),
                signer_acct_id: Self::get_from_row_with_header(SIGNER_ACCT_ID_HEADER, header_map, row).parse().expect("Should parse to integer."),
                rad_last_name: Self::get_from_row_with_header(RAD_LAST_NAME_HEADER, header_map, row),
                rad_first_name: Self::get_from_row_with_header(RAD_FIRST_NAME_HEADER, header_map, row),
                list_time: datetime,
                rvu: Self::get_from_row_with_header(RVU_HEADER, header_map, row).parse().expect("Should parse to float."),
                site_id: Self::get_from_row_with_header(SITE_ID_HEADER, header_map, row).parse().expect("Should parse to integer."),
            }
        )
    }
}

impl ExamTable {

}