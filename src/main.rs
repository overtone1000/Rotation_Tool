use std::{error::Error, fs::{self, File}, collections::HashMap, io::{Write, BufWriter}};

use categorization::exam_categories::exam_category;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, Datelike, Timelike};
use constraints::{ConstraintSet, is_not_holiday, is_weekday, exclude_site, is_business_day};
use globals::{main_headers, NEURO_BRAIN, NEURO_OTHER, MSK, Outpatient, TPC};
use processed_source::ProcessedSource;
use rvu_map::{RVUMap, MapCoords, buildMaps};
use table::Table;
use explain::*;

use crate::{globals::file_names::{self, SOURCE_CACHE, COVERAGE_AUDIT_OUT, COVERAGE_ANALYSIS_OUT}, error::RotationToolError, categorization::{buildSalemRVUMap, get_categories_list, get_locations_list, backup, buildSalemBVUMap}, time::{getTimeRowNormalDistWeights, getNormalDistWeights}, analysis::coverage_tree::{CoverageMap, CoordinateMap, CoverageCoordinates}, rotations::time_modifiers::TimeSinceMidnight};

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

    println!("Adding coverage.")   ;
    coverage_tree.add_coverage_from_manifest(manifest)?;

    /*
    let test = coverage_tree.get_map().get_mut("SH").expect("Testing")
        .get_map().get_mut("Neuro (Other)").expect("Testing")
        .get_map().get_mut("ED").expect("Testing")
        .get_map().get_mut("MR").expect("Testing");
    */  

    let source= ProcessedSource::load_from_cache(SOURCE_CACHE)?;
    println!("Finished loading source from cache.");

    println!("Adding work to tree.");
    coverage_tree.add_work_from_source(source, &date_constraint_set)?;


    let auditfile=File::create(COVERAGE_AUDIT_OUT)?;
    let mut writer = BufWriter::new(auditfile);
    coverage_tree.audit_to_stream(&mut writer)?;

    coverage_tree.analysis_to_file(COVERAGE_ANALYSIS_OUT.to_owned()+"_rvu.csv",true);
    coverage_tree.analysis_to_file(COVERAGE_ANALYSIS_OUT.to_owned()+"_bvu.csv",false);

    Ok(())
}

fn cache_source()->Result<(), Box<dyn Error>> {
    println!("Processing source.");
    let source= match ProcessedSource::build()
    {
        Ok(x)=>x,
        Err(e)=>{return Err(e);}
    };

    let retval = source.save_to_cache(SOURCE_CACHE);
    println!("Finished caching source.");
    retval
}

fn main()->Result<(), Box<dyn Error>> {
    print!("{}[2J", 27 as char);
    for _ in 1..10
    {
        println!();
    }
    println!("Starting.");

    let rebuild_source:bool=false;

    if rebuild_source
    {
        cache_source()?;
    }

    let retval=analyze_rotations();
    
    println!("Finished.");
    retval
}

