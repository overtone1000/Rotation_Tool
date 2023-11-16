use std::{error::Error, fs::{self, File}, collections::HashMap, io::Write};

use categorization::exam_categories::exam_category;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Datelike, Timelike};
use constraints::{ConstraintSet, is_not_holiday, is_weekday, exclude_site, is_business_day};
use globals::{main_headers, NEURO_BRAIN, NEURO_OTHER, MSK, Outpatient, TPC};
use processed_source::ProcessedSource;
use rvu_map::{RVUMap, MapCoords, buildMaps};
use table::Table;
use explain::*;

use crate::{globals::file_names, error::RotationToolError, categorization::{buildSalemRVUMap, get_categories_list, get_locations_list, backup, buildSalemBVUMap}, time::{getTimeRowNormalDistWeights, getNormalDistWeights}};

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
mod rotations;
mod analysis;

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
    
    //crate::rotations::manifest:: Manifest::create_example();

    let filename="./rotations/active.yaml";
    let manifest=match crate::rotations::manifest::Manifest::parse(filename)
    {
        Ok(x)=>x,
        Err(e)=>{return Err(e);}
    };

    let source= match ProcessedSource::build()
    {
        Ok(x)=>x,
        Err(e)=>{return Err(e);}
    };

    let mut date_constraint_set:ConstraintSet<NaiveDateTime>=ConstraintSet::new();
    date_constraint_set.add(&is_not_holiday);
    let converage_tree=match analysis::coverage_tree::CoverageTree::build(source, &date_constraint_set)
    {
        Ok(x)=>x,
        Err(e)=>{return Err(e);}
    };
    
    Ok(())
}


fn main()->Result<(), Box<dyn Error>> {
    println!("Starting.");

    let retval=analyze_rotations();
    
    println!("Finished.");
    retval
}

