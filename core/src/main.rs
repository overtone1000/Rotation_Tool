use std::{
    error::Error,
    fs::File,
    io::BufWriter,
};


use chrono::NaiveDateTime;
use constraints::{is_business_day, is_not_holiday, ConstraintSet};


use processed_source::ProcessedSource;


use crate::{
    analysis::coverage_tree::CoverageMap,
    globals::file_names::{COVERAGE_ANALYSIS_OUT, COVERAGE_AUDIT_OUT, SOURCE_CACHE},
};

mod analysis;
mod categorization;
mod constraints;
mod dates;
mod error;
mod explain;
mod globals;
mod processed_source;
mod rotations;
mod rvu_map;
mod table;
mod time;
mod tpc;

fn build_maps() -> Result<(), Box<dyn Error>> {
    let date_constraints = is_business_day();

    //explain_weekday_variance()

    //let exclude_tpc_ref = &exclude_site(TPC.to_string());

    //let mut ccs:ConstraintSet<MapCoords>=ConstraintSet::new();
    //ccs.add(exclude_tpc_ref);

    //buildMaps(Some(ccs))

    rvu_map::build_maps(&date_constraints, None)
}

fn parse_manifest() -> Result<rotations::manifest::Manifest, Box<dyn Error>> {
    println!("Parsing manifest.");
    let filename = "./rotations/active.yaml";
    crate::rotations::manifest::Manifest::parse(filename)
}

fn analyze_rotations() -> Result<(), Box<dyn Error>> {
    //crate::rotations::manifest:: Manifest::create_example();

    let manifest: rotations::manifest::Manifest = parse_manifest()?;

    println!("Building coverage tree.");
    let mut date_constraint_set: ConstraintSet<NaiveDateTime> = ConstraintSet::new();
    date_constraint_set.add(&is_not_holiday);

    let mut coverage_tree = CoverageMap::default();

    println!("Adding coverage.");
    coverage_tree.add_coverage_from_manifest(manifest)?;

    /*
    let test = coverage_tree.get_map().get_mut("SH").expect("Testing")
        .get_map().get_mut("Neuro (Other)").expect("Testing")
        .get_map().get_mut("ED").expect("Testing")
        .get_map().get_mut("MR").expect("Testing");
    */

    let source = ProcessedSource::load_from_cache(SOURCE_CACHE)?;
    println!("Finished loading source from cache.");

    println!("Adding work to tree.");
    coverage_tree.add_work_from_source(source, &date_constraint_set)?;

    let auditfile = File::create(COVERAGE_AUDIT_OUT)?;
    let mut writer = BufWriter::new(auditfile);
    coverage_tree.audit_to_stream(&mut writer)?;

    coverage_tree.analysis_to_file(COVERAGE_ANALYSIS_OUT.to_owned() + "_rvu.csv", true);
    coverage_tree.analysis_to_file(COVERAGE_ANALYSIS_OUT.to_owned() + "_bvu.csv", false);

    Ok(())
}

fn cache_source() -> Result<(), Box<dyn Error>> {
    println!("Processing source.");
    let source = match ProcessedSource::build() {
        Ok(x) => x,
        Err(e) => {
            return Err(e);
        }
    };

    let retval = source.save_to_cache(SOURCE_CACHE);
    println!("Finished caching source.");
    retval
}

fn main() -> Result<(), Box<dyn Error>> {
    print!("{}[2J", 27 as char);
    for _ in 1..10 {
        println!();
    }
    println!("Starting.");

    let rebuild_source: bool = false;

    if rebuild_source {
        cache_source()?;
    }

    let rotation_analysis:bool = false;

    if rotation_analysis{
        analyze_rotations()?;
    }

    let manifest = parse_manifest()?;
    manifest.to_json("../frontend/static/active.json")?;

    println!("Finished.");
    Ok(())
}
