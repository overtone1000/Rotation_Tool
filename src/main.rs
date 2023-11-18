use std::{error::Error, fs::{self, File}, collections::HashMap, io::Write};

use categorization::exam_categories::exam_category;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Datelike, Timelike};
use constraints::{ConstraintSet, is_not_holiday, is_weekday, exclude_site, is_business_day};
use globals::{main_headers, NEURO_BRAIN, NEURO_OTHER, MSK, Outpatient, TPC};
use processed_source::ProcessedSource;
use rvu_map::{RVUMap, MapCoords, buildMaps};
use table::Table;
use explain::*;

use crate::{globals::file_names, error::RotationToolError, categorization::{buildSalemRVUMap, get_categories_list, get_locations_list, backup, buildSalemBVUMap}, time::{getTimeRowNormalDistWeights, getNormalDistWeights}, analysis::coverage_tree::CoverageMap};

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

    println!("Parsing manifest.");
    let filename="./rotations/active.yaml";
    let manifest=match crate::rotations::manifest::Manifest::parse(filename)
    {
        Ok(x)=>x,
        Err(e)=>{return Err(e);}
    };

    println!("Building coverage tree.");
    let mut date_constraint_set:ConstraintSet<NaiveDateTime>=ConstraintSet::new();
    date_constraint_set.add(&is_not_holiday);
    
    let mut coverage_tree=CoverageMap::default();
        
    match coverage_tree.add_coverage_from_manifest(manifest)
    {
        Ok(x)=>x,
        Err(e)=>{return Err(e);}
    };


    println!("Processing source.");
    let source= match ProcessedSource::build()
    {
        Ok(x)=>x,
        Err(e)=>{return Err(e);}
    };

    println!("Adding work to tree.");
    match coverage_tree.add_work_from_source(source, &date_constraint_set)
    {
        Ok(x)=>x,
        Err(e)=>{return Err(e);}
    };

    let coverage_errors=coverage_tree.audit();
    for ce in coverage_errors
    {
        println!("{ce}");
    }

    Ok(())
}


fn main()->Result<(), Box<dyn Error>> {
    println!("Starting.");

    let retval=analyze_rotations();
    
    println!("Finished.");
    retval
}

