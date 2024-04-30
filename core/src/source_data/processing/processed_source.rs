use std::{
    collections::BTreeMap,
    error::Error,
};

use crate::{
    globals::file_names,
    source_data::tables::{bvu_map::BVUMap, exam_aliases::Exam_Aliases, exam_categories::Exam_Categories, exam_data::{Exam, ExamTable}, location_categories::Location_Categories, readers::{ExamReader, ReaderTable}, table::Table, types::{Context, ExamCode, Location, Subspecialty}},
};

use super::categorization::{check_bvusource, check_categories_list, check_readers, get_site_and_location_context_map};

pub struct ProcessedSource {
    pub main_data: Vec<Exam>,
    pub bvu_map: BTreeMap<ExamCode,f64>,
    pub subspecialty_map: BTreeMap<ExamCode,Subspecialty>,
    pub context_map: BTreeMap<u64,BTreeMap<Location,Context>>,
    pub alias_map: BTreeMap<ExamCode,String>,
    pub readers:BTreeMap<u64,ExamReader>
}

impl ProcessedSource {
    pub fn build() -> Result<ProcessedSource, Box<dyn Error>> {
        let bvu_data_table = BVUMap::create(file_names::BVU_DATA_FILE);
        let exam_categories_table=Exam_Categories::create(file_names::CATEGORIES_EXAM_FILE);

        let mut bvu_map:BTreeMap<ExamCode,f64>=BTreeMap::new();
        for bvu_entry in bvu_data_table.iter()
        {
            bvu_map.insert(
                bvu_entry.exam_code,
                bvu_entry.bvu
            );
        }

        let mut subspecialty_map: BTreeMap<ExamCode,Subspecialty> = BTreeMap::new();        
        for exam_category in exam_categories_table.iter() {
            subspecialty_map.insert(
                exam_category.exam_code,
                exam_category.subspecialty,
            );
        }

        let mut alias_map:BTreeMap<String,ExamCode> = BTreeMap::new();
        let alias_table=Exam_Aliases::create(file_names::EXAM_ALIAS_FILE);        
        for alias in alias_table.iter()
        {
            match alias_map.insert(
                alias.alias,
                alias.exam_code
            )
            {
                Some(x)=>panic!("Duplicate alias {:?}",x),
                _=>()
            }
        }

        let mut reader_map:BTreeMap<u64,ExamReader>=BTreeMap::new();
        for reader in ReaderTable::create(file_names::READERS_FILE).iter()
        {
            reader_map.insert(reader.signer_acct_id,reader);
        }
        
        let mut main_data:Vec<Exam> = ExamTable::create(file_names::MAIN_DATA_FILE).get_from_cache_or_build_and_cache()?;
        
        //Substitute aliases and check readers
        for exam in &mut main_data
        {
            match alias_map.get(&exam.exam_code)
            {
                Some(val)=>{exam.exam_code=val.to_string()},
                _=>()
            }
        }
        //Checks should happen after alias substitution
        check_categories_list(&main_data, &exam_categories_table)?;
        check_bvusource(&main_data,&bvu_data_table)?;
        check_readers(&main_data,&reader_map)?;

        Ok(ProcessedSource {
            main_data: main_data,
            bvu_map: bvu_map,
            subspecialty_map: subspecialty_map,
            context_map:get_site_and_location_context_map(&Location_Categories::create(file_names::CATEGORIES_LOCATION_FILE))?,
            alias_map: alias_map,
            readers:reader_map
        })
    }
}
