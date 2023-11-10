use std::{error::Error, fs::{self, File}, collections::HashMap, io::Write};

use categorization::exam_categories::exam_category;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Datelike, Timelike};
use constraints::{ConstraintSet, is_not_holiday, is_weekday, exclude_site, is_business_day};
use globals::{main_headers, NEURO_BRAIN, NEURO_OTHER, MSK, Outpatient, TPC};
use rotation_descriptions::RotationDescriptionsDocument;
use rvu_map::{RVUMap, MapCoords, buildMaps};
use table::Table;
use explain::*;

use crate::{globals::file_names, error::RotationToolError, categorization::{buildSalemRVUMap, get_categories_list, get_locations_list, backup, buildSalemBVUMap}, time::getTimeRowNormalDistWeights};

mod globals;
mod error;
mod time;
mod table;
mod dates;
mod rvu_map;
mod tpc;
mod categorization;
mod constraints;
mod explain;
mod processed_source;
mod rotation_descriptions;

fn build_maps()->Result<(), Box<dyn Error>> {
    let date_constraints = is_business_day();
    
    //explain_weekday_variance()

    //let exclude_tpc_ref = &exclude_site(TPC.to_string());

    //let mut ccs:ConstraintSet<MapCoords>=ConstraintSet::new();
    //ccs.add(exclude_tpc_ref);

    //buildMaps(Some(ccs))

    buildMaps(&date_constraints, None)
}


fn analyze_rotations()->Result<(), Box<dyn Error>> {
    
    RotationDescriptionsDocument::create_example();
    //RotationDescriptionsDocument::parse(filename);

    Ok(())
}


fn main()->Result<(), Box<dyn Error>> {
    println!("Starting.");

    let retval=analyze_rotations();
    
    println!("Finished.");
    retval
}

