use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufWriter, Write},
};

use analysis::analysis_datum::AnalysisDatum;
use chrono::{NaiveDate, NaiveDateTime, Weekday};
use constraints::{is_not_holiday, ConstraintSet};

use coverage::{
    analysis::{
        by_day_of_week::{analysis_to_csv, analyze_by_day_of_week},
        coverage_audit::{audit, audit_to_stream},
    }, coordinate::CoverageCoordinates, coverage_and_work_day::CoverageAndWorkDay, units::CoverageUnit, work_coverage_map::maps::CoverageMap
};
use globals::file_names::COVERAGE_AUDIT_NOWORK_OUT;

use crate::{
    coverage::analysis::{
        rotation_day_details::details,
        volumes_by_date::{analysis_to_plot, sort_volumes_by_date},
    },
    globals::file_names::{
        self, COVERAGE_ANALYSIS_OUT, COVERAGE_AUDIT_OUT, SOURCE_CACHE, VOLUME_BY_DATE_ROTATION
    },
    serialization::output::JSONFileOut,
    source_data::{processing::processed_source::ProcessedSource, tables::exam_categories::Exam_Categories},
};

mod analysis;
mod constraints;
mod coverage;
mod dates;
mod error;
mod globals;
mod rotations;
mod serialization;
mod source_data;

pub struct MainCommon {
    pub coverage_tree: CoverageMap,
    pub source: ProcessedSource,
}

pub fn build_main_common() -> Result<MainCommon, Box<dyn Error>> {
    let source = ProcessedSource::build()?;

    let manifest: rotations::manifest::Manifest = parse_manifest()?;

    println!("Building coverage tree.");
    let mut date_constraint_set: ConstraintSet<NaiveDateTime> = ConstraintSet::new();
    date_constraint_set.add(&is_not_holiday);

    let mut coverage_tree = CoverageMap::default();

    println!("Adding coverage.");
    coverage_tree.add_coverage_from_manifest(manifest)?;


    println!("Adding work to tree.");
    coverage_tree.add_work_from_source(&source, &date_constraint_set)?;

    Ok(MainCommon {
        coverage_tree,
        source,
    })
}

fn parse_manifest() -> Result<rotations::manifest::Manifest, Box<dyn Error>> {
    println!("Parsing manifest.");
    let filename = "./rotations/active.yaml";
    crate::rotations::manifest::Manifest::parse(filename)
}

pub fn analyze_rotations(
    common: &mut MainCommon,
) -> Result<HashMap<String, HashMap<Weekday, AnalysisDatum>>, Box<dyn Error>> {
    let auditfile: File = File::create(COVERAGE_AUDIT_OUT)?;
    let mut writer = BufWriter::new(auditfile);

    let auditfile_nowork: File = File::create(COVERAGE_AUDIT_NOWORK_OUT)?;
    let mut writer_nowork = BufWriter::new(auditfile_nowork);

    //let audit_result = audit(&mut common.coverage_tree);
    //audit_to_stream(&audit_result, &mut writer, &mut writer_nowork)?;

    let analysis = analyze_by_day_of_week(&mut common.coverage_tree);
    analysis_to_csv(
        &analysis,
        COVERAGE_ANALYSIS_OUT.to_owned() + "_rvu.csv",
        true,
    );
    analysis_to_csv(
        &analysis,
        COVERAGE_ANALYSIS_OUT.to_owned() + "_bvu.csv",
        false,
    );

    Ok(analysis)
}

pub fn generate_frontend_statics(common: &mut MainCommon) -> Result<(), Box<dyn Error>> {
    let base: &str = "../frontend/static/data";
    let millistr = chrono::Local::now().timestamp_millis().to_string();

    let mut file = File::create("../frontend/src/commons/key.ts")?;
    writeln!(
        file,
        "//This file is automatically generated by the rust core! Please do not change."
    )?;
    writeln!(file, "export let key={}", millistr)?;

    std::fs::remove_dir_all(base)?;
    std::fs::create_dir(base)?;

    let manifest = parse_manifest()?;
    manifest.to_json(&(base.to_string() + "/active_rotation_manifest" + &millistr + ".json"))?;

    let mut coverage_tree = CoverageMap::default();
    println!("Adding coverage.");
    coverage_tree.add_coverage_from_manifest(manifest)?;
    coverage_tree.to_json(&(base.to_string() + "/active_coverage_tree" + &millistr + ".json"))?;

    //Categories lists
    common.source.subspecialty_map
        .to_json(&(base.to_string() + "/exam_categories" + &millistr + ".json"))?;

    //Plots
    let mut plot = sort_volumes_by_date(&mut common.coverage_tree);
    analysis_to_plot(
        &mut plot,
        base.to_string() + "/" + VOLUME_BY_DATE_ROTATION + &millistr + ".json",
    )?;

    Ok(())
}

pub fn perform_detailed_analysis(common: &mut MainCommon) -> Result<(), Box<dyn Error>> {
    detailed_analysis(common, chrono::Weekday::Mon, globals::MSK)?;
    detailed_analysis(common, chrono::Weekday::Sun, globals::MSK_WE_AH0C)?;
    Ok(())
}

fn detailed_analysis(
    common: &mut MainCommon,
    weekday: chrono::Weekday,
    rotation: &str,
) -> Result<(), Box<dyn Error>> {
    let details = details(&mut common.coverage_tree, weekday, rotation)?;

    println!("Detailed analysis for {}-{}", rotation, weekday);
    for (exam, count) in details.get_studies() {
        println!("{},{}", exam, count);
    }

    Ok(())
}