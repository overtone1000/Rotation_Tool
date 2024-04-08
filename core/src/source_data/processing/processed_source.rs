use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    fs::File,
    io::{BufWriter, Read},
};

use serde::{Deserialize, Serialize};

use crate::{
    globals::file_names,
    source_data::{processing::categorization::{
    }, tables::{bvu_map::{BVUMap, BVUMapEntry}, exam_categories::{self, ExamCategoryEntry, Exam_Categories}, exam_data::{Exam, ExamTable}, location_categories::{LocationCategoryEntry, Location_Categories}, table::Table, types::{Context, ExamCode, Location, Subspecialty}}},
};

use super::categorization::{check_bvusource, check_categories_list, get_site_and_location_context_map};

pub struct ProcessedSource {
    pub main_data: ExamTable,
    pub bvu_map: BTreeMap<ExamCode,f64>,
    pub subspecialty_map: BTreeMap<ExamCode,Subspecialty>,
    pub context_map: BTreeMap<u64,BTreeMap<Location,Context>>,
}

impl ProcessedSource {
    pub fn build() -> Result<ProcessedSource, Box<dyn Error>> {
        let main_data_table = ExamTable::create(file_names::MAIN_DATA_FILE);
        let bvu_data_table = BVUMap::create(file_names::BVU_DATA_FILE);
        let exam_categories_table=Exam_Categories::create(file_names::CATEGORIES_EXAM_FILE);

        check_categories_list(&main_data_table, &exam_categories_table)?;
        check_bvusource(&main_data_table,&bvu_data_table);

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

        Ok(ProcessedSource {
            main_data: main_data_table,
            bvu_map: bvu_map,
            subspecialty_map: subspecialty_map,
            context_map:get_site_and_location_context_map(&Location_Categories::create(file_names::CATEGORIES_LOCATION_FILE))?
        })
    }
}
