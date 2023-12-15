use std::{
    error::Error,
    fs::File,
    io::BufWriter, os::unix::process,
};


use chrono::NaiveDateTime;
use constraints::{is_business_day, is_not_holiday, ConstraintSet};


use processed_source::ProcessedSource;


use crate::{
    analysis::coverage_tree::CoverageMap,
    globals::file_names::{COVERAGE_ANALYSIS_OUT, COVERAGE_AUDIT_OUT, SOURCE_CACHE}, rotations::manifest::JSONable,
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

struct ProcessedData
{
    pub coverage_tree:CoverageMap,
    pub source:ProcessedSource,
}
fn process_data() -> Result<ProcessedData, Box<dyn Error>> {
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
    coverage_tree.add_work_from_source(&source, &date_constraint_set)?;

    Ok(
        ProcessedData{
            coverage_tree:coverage_tree,
            source:source
        }
    )
}

fn analyze_rotations() -> Result<(), Box<dyn Error>> {
    let mut processed_data = process_data()?;

    let auditfile: File = File::create(COVERAGE_AUDIT_OUT)?;
    let mut writer = BufWriter::new(auditfile);

    processed_data.coverage_tree.audit_to_stream(&mut writer)?;

    processed_data.coverage_tree.analysis_to_file(COVERAGE_ANALYSIS_OUT.to_owned() + "_rvu.csv", true);
    processed_data.coverage_tree.analysis_to_file(COVERAGE_ANALYSIS_OUT.to_owned() + "_bvu.csv", false);

    Ok(())
}

fn detailed_analysis(processed_data:&mut ProcessedData, weekday:chrono::Weekday, rotation:&str) -> Result<(), Box<dyn Error>> {
    let details = processed_data.coverage_tree.details(weekday,rotation)?;

    println!("Detailed analysis for {}-{}",rotation,weekday);
    for (exam,count) in details.get_studies()
    {
        println!("{},{}",exam,count);
    }

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

    let output_to_json:bool = true;
    if output_to_json
    {
        let manifest = parse_manifest()?;
        manifest.to_json("../frontend/static/active_rotation_manifest.json")?;
        
        let mut coverage_tree = CoverageMap::default();
        println!("Adding coverage.");
        coverage_tree.add_coverage_from_manifest(manifest)?;
        coverage_tree.to_json("../frontend/static/active_coverage_tree.json")?;

        let source = ProcessedSource::load_from_cache(SOURCE_CACHE)?;

        source.exam_categories_list.to_json("../frontend/static/exam_categories.json")?;
    }

    let perform_detailed_analysis:bool = false;
    if perform_detailed_analysis
    {
        let mut processed_data = process_data()?;
        detailed_analysis(&mut processed_data,chrono::Weekday::Mon, globals::MSK)?;
        detailed_analysis(&mut processed_data,chrono::Weekday::Sun, globals::MSK_WE_AH0C)?;
    }

    println!("Finished.");
    Ok(())
}
