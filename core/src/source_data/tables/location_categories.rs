use std::{collections::{HashMap, HashSet}, error::Error, num::ParseIntError};

use chrono::{NaiveDateTime};
use serde::Deserialize;

use super::{table::Table, types::{Context, Location}};

pub struct LocationCategoryEntry {
    pub site_id:u64,
    pub location:Location,
    pub context:Context,
}

impl Eq for LocationCategoryEntry {}

    impl PartialOrd for LocationCategoryEntry {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for LocationCategoryEntry {
        fn eq(&self, other: &Self) -> bool {
            self.location == other.location
            //self.context == other.context &&
            //self.comments == other.comments
        }
    }

    impl Ord for LocationCategoryEntry {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.location.cmp(&other.location)
        }
    }

const SITE_HEADER:&str="Site";
const LOCATION_HEADER:&str="Location";
const CONTEXT_HEADER:&str="Context";

pub struct Location_Categories {
    filename:String
}

impl Table<LocationCategoryEntry> for Location_Categories
{
    fn get_file_path(&self)->&str {&self.filename}

    fn build_from_headers_and_row(header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<LocationCategoryEntry, Box<dyn std::error::Error>>{
        Ok(
            LocationCategoryEntry{
                site_id:Self::get_from_row_with_header(SITE_HEADER, header_map, row).parse()?,
                location: Self::get_from_row_with_header(LOCATION_HEADER, header_map, row),
                context: Self::get_from_row_with_header(CONTEXT_HEADER, header_map, row),
            }
        )
    }
}

impl Location_Categories {
    pub fn create(filename:&str)->Location_Categories{Location_Categories{filename:filename.to_string()}}
}