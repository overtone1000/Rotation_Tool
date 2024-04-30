use std::collections::{HashMap, HashSet};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::{table::Table, types::{ExamCode, ExamDescription, Location}};

#[derive(Debug,Serialize,Deserialize)]
pub struct Exam {
    pub accession:String, //Accession
    pub exam_code:ExamCode, //ProcedureCodeList
    pub procedure_description:ExamDescription, //ProcedureDescList
    pub signer_acct_id:u64, //SignerAcctID
    pub rad_last_name:String, //RadLastNm
    pub rad_first_name:String, //RadFirstNm
    pub list_datetime:NaiveDateTime, //Exam Started
    pub rvu:f64, //WorkRVU
    pub site_id:u64, //SiteID
    pub location:Location,
    pub class:u64, //PatientClassID
}

const ACCESSION_HEADER:&str="Accession";
const PROCEDURE_CODE_HEADER:&str="ProcedureCodeList";
const PROCEDURE_DESCRIPTION_HEADER:&str="ProcedureDescList";
const SIGNER_ACCT_ID_HEADER:&str="SignerAcctID";
const RAD_LAST_NAME_HEADER:&str="RadLastNm";
const RAD_FIRST_NAME_HEADER:&str="RadFirstNm";
const LIST_TIME_HEADER:&str="Exam Started";
const RVU_HEADER:&str="WorkRVU";
const SITE_ID_HEADER:&str="SiteID";
const LOCATION_HEADER:&str="LocationDescription";
const PATIENT_CLASS_HEADER:&str="PatientClassID";

pub struct ExamTable {
    filename:String
}

impl Table<Exam> for ExamTable
{
    fn get_file_path(&self)->&str {&self.filename}

    fn build_from_headers_and_row(header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<Exam, Box<dyn std::error::Error>>{

        Ok(
            Exam{
                accession:Self::get_from_row_with_header(ACCESSION_HEADER, header_map, row),
                exam_code: Self::get_from_row_with_header(PROCEDURE_CODE_HEADER, header_map, row),
                procedure_description: Self::get_from_row_with_header(PROCEDURE_DESCRIPTION_HEADER, header_map, row),
                signer_acct_id: Self::parse(SIGNER_ACCT_ID_HEADER,header_map,row)?,
                rad_last_name: Self::get_from_row_with_header(RAD_LAST_NAME_HEADER, header_map, row),
                rad_first_name: Self::get_from_row_with_header(RAD_FIRST_NAME_HEADER, header_map, row),
                list_datetime: Self::get_as_date(LIST_TIME_HEADER,header_map,row)?,
                rvu: Self::parse(RVU_HEADER,header_map,row)?,
                site_id: Self::parse(SITE_ID_HEADER,header_map,row)?,
                location: Self::get_from_row_with_header(LOCATION_HEADER, header_map, row),
                class: Self::parse(PATIENT_CLASS_HEADER, header_map, row)?
            }
        )
    }
}

#[derive(Debug,Serialize,Deserialize)]
struct ExamTableCache
{
    file_timestamp:std::time::SystemTime,
    data:Vec<Exam>
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
    pub fn get_from_cache_or_build_and_cache(&self)->Result<Vec<Exam>,Box<dyn std::error::Error>>
    {
        println!("Checking for cache.");
        let last_modified_time=match std::fs::File::open(self.get_file_path())
        {
            Ok(data_file) => {
                match std::fs::File::metadata(&data_file)
                {
                    Ok(file_data) => {
                        match file_data.modified()
                        {
                            Ok(last_modification) => {
                                last_modification
                            },
                            Err(e) => {
                                return Err(Box::new(e));
                            }
                        }
                    },
                    Err(e) => {
                        return Err(Box::new(e));
                    }
                }
            },
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        let cache:Option<ExamTableCache> = match std::fs::File::open(crate::globals::file_names::SOURCE_CACHE)
        {
            Ok(cache_file)=>{
                let reader=std::io::BufReader::new(cache_file);
                let read_result:Result<ExamTableCache,serde_json::Error> =serde_json::from_reader(reader);
                let result=match read_result {
                    Ok(deserialized)=>{
                        if last_modified_time==deserialized.file_timestamp
                        {
                            Some(deserialized)
                        }
                        else {
                            None
                        }
                    },
                    Err(e)=>{None}
                };
                result
            }
            Err(e)=>{
                None
            }
        };

        match cache
        {
            Some(cache)=>{
                println!("Using cache.");
                Ok(cache.data)
            },
            None=>{
                println!("Updating cache.");
                let new_cache = ExamTableCache{
                    file_timestamp:last_modified_time,
                    data:self.iter().collect()
                };
                let cache_file= std::fs::File::create(crate::globals::file_names::SOURCE_CACHE)?;
                let writer=std::io::BufWriter::new(cache_file);
                serde_json::to_writer(writer, &new_cache)?;
                Ok(new_cache.data)
            }
        }
    }
}