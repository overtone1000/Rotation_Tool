use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufWriter, Read},
};

use serde::{Deserialize, Serialize};

use crate::{
    globals::file_names,
    source_data::{processing::categorization::{
    }, tables::{bvu_map::{BVUMap, BVUMapEntry}, exam_categories::{ExamCategoryEntry, Exam_Categories}, exam_data::{Exam, ExamTable}, location_categories::{LocationCategoryEntry, Location_Categories}, table::Table}},
};

use super::categorization::{check_categories_list, get_site_and_location_context_map};

pub struct ProcessedSource {
    pub main_data_table: ExamTable,
    pub bvu_data_table: Vec<BVUMapEntry>,
    pub exam_categories_table: Vec<ExamCategoryEntry>,
    pub location_categories_table: Vec<LocationCategoryEntry>,
    pub exam_categories_list: Vec<ExamCategoryEntry>,
    pub exam_to_subspecialty_map: HashMap<String, String>,
    pub site_and_location_to_context_map: HashMap<u64, HashMap<String, String>>,
}

impl ProcessedSource {
    pub fn build() -> Result<ProcessedSource, Box<dyn Error>> {
        let main_data_table = ExamTable::create(file_names::MAIN_DATA_FILE);
        let bvu_data_table = BVUMap::create(file_names::BVU_DATA_FILE).collect();
        let mut exam_categories_table = Exam_Categories::create(file_names::CATEGORIES_EXAM_FILE).collect();
        let mut location_categories_table = Location_Categories::create(file_names::CATEGORIES_LOCATION_FILE).collect();
        let exam_categories_list = check_categories_list(&main_data_table, &exam_categories_table)?;
        let mut exam_to_subspecialty_map: HashMap<String, String> = HashMap::new();
        let mut site_and_location_to_context_map: HashMap<u64, HashMap<String, String>>=get_site_and_location_context_map(&location_categories_table)?;

        exam_categories_table.clear();
        for category_row in exam_categories_list.as_slice() {
            let mut newrow: Vec<String> = Vec::new();
            newrow.push(category_row.procedure_code.to_owned());
            newrow.push(category_row.exam.to_owned());
            newrow.push(category_row.subspecialty.to_owned());
            newrow.push(category_row.comments.to_owned());
            exam_categories_table.pushrow(newrow);
        }

        for exam_category in &exam_categories_list {
            exam_to_subspecialty_map.insert(
                exam_category.procedure_code.to_string(),
                exam_category.subspecialty.to_string(),
            );
        }

        Ok(ProcessedSource {
            main_data_table,
            bvu_data_table,
            exam_categories_table,
            location_categories_table,
            exam_categories_list,
            exam_to_subspecialty_map,
            site_and_location_to_context_map,
        })
    }

    /*
    pub fn save_to_cache(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        println!("Saving processed source to cache.");
        let cachefile = File::create(filename)?;
        let writer = BufWriter::new(cachefile);
        let mut serializer = serde_json::Serializer::new(writer);

        Ok(self.serialize(&mut serializer)?)
    }

    pub fn load_from_cache(filename: &str) -> Result<ProcessedSource, Box<dyn Error>> {
        println!("Reading processed source from cache.");
        let mut cachefile = File::open(filename)?;
        //let reader = BufReader::new(cachefile); //Still slow! They're not big enough. Just load straight to memory.
        let mut str = String::new();
        match cachefile.read_to_string(&mut str) {
            Err(e) => {
                return Err(Box::new(e));
            }
            Ok(_) => (),
        };
        let mut deserializer = serde_json::Deserializer::from_str(&str);

        Ok(ProcessedSource::deserialize(&mut deserializer)?)
    }
    */

    pub fn check_bvusource(&mut self) {
        //This was only necessary to fix the data.
        crate::source_data::processing::categorization::check_bvusource(
            &self.main_data_table,
            &mut self.bvu_data_table,
        );
    }
}
