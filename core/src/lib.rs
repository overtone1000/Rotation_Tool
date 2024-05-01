use std::{
    collections::{BTreeMap, BTreeSet}, error::Error, fs::File, io::BufWriter
};

use chrono::{NaiveDate, NaiveDateTime};
use constraints::{is_not_holiday, ConstraintSet};

use coverage::{
    analysis::{
        by_day_of_week::{analysis_to_csv, analyze_by_day_of_week},
        coverage_audit::{audit, audit_to_stream},
    }, work_coverage_map::maps::CoverageMap
};
use globals::file_names::{ACTIVE_COVERAGE_ANALYSIS_OUT, ACTIVE_COVERAGE_AUDIT_NOWORK_OUT, ACTIVE_COVERAGE_AUDIT_OUT, MANIFEST_ACTIVE, MANIFEST_PROPOSED, VOLUME_BY_DATE_ROTATION_PROPOSED};
use rotations::manifest::Manifest;

use crate::{
    coverage::analysis::{
        comparison::compare, rotation_day_details::details, volumes_by_rotation_date::{analysis_to_plot, sort_volumes_by_rotation_date}, volumes_by_site_date::{sort_volumes_by_facility_and_date, volumes_by_facility_and_date_to_plot}
    },
    globals::file_names::{
        PROPOSED_COVERAGE_ANALYSIS_OUT, PROPOSED_COVERAGE_AUDIT_NOWORK_OUT, PROPOSED_COVERAGE_AUDIT_OUT, PROPOSED_DIFFERENTIAL, VOLUME_BY_DATE_FACILITY, VOLUME_BY_DATE_ROTATION_ACTIVE
    },
    serialization::output::JSONFileOut,
    source_data::processing::processed_source::ProcessedSource,
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

pub enum ManifestType
{
    Active,
    Proposed
}

impl ManifestType
{
    pub fn get(&self)->Result<rotations::manifest::Manifest, Box<dyn Error>>
    {
        let filename = match self
        {
            ManifestType::Active=>MANIFEST_ACTIVE,
            ManifestType::Proposed=>MANIFEST_PROPOSED
        };
        crate::rotations::manifest::Manifest::parse(filename)
    }
}

pub fn build_main_common() -> Result<MainCommon, Box<dyn Error>> {
    let source = ProcessedSource::build()?;

    let manifest=ManifestType::Active.get()?;
    let coverage_tree=build_coverage_tree_from_manifest(manifest,&source)?;

    Ok(MainCommon {
        coverage_tree,
        source
    })
}

pub fn print_averages_by_modality_and_day(map:&CoverageMap, rotation_start:&NaiveDate, rotation_end:&NaiveDate) -> () {
    let mut dates:BTreeSet<NaiveDate>=BTreeSet::new();
    let mut aggregate:BTreeMap<String,(f64,f64)>=BTreeMap::new();
    map.foreach(
        |coord,cawd|
        {
            cawd.for_each_analysis_datum_by_rotation_date(
                |date,datum,_cu|
                {
                    if date>=*rotation_start && date<=*rotation_end
                    {
                        dates.insert(date);
                        
                        let value = match aggregate.entry(coord.subspecialty.to_string())
                        {
                            std::collections::btree_map::Entry::Vacant(e) => e.insert((0.0,0.0)),
                            std::collections::btree_map::Entry::Occupied(e) => e.into_mut(),
                        };

                        let mut total_study_count:f64=0.0;
                        for study_count in datum.get_studies().values()
                        {
                            total_study_count+=study_count;
                        }

                        (*value).0+=total_study_count;
                        (*value).1+=datum.get_rvu();
                    }
                }
            );
        }
    );

    println!();
    println!("Averages");
    println!("Subspecialty,Number of Studies,RVUs");
    let denominator=f64::from(dates.len() as u32);
    for (subspecialty,(study_count,rvus)) in aggregate
    {
        let average_number_of_studies=study_count/denominator;
        let average_rvus=rvus/denominator;
        println!("{},{},{}",subspecialty,average_number_of_studies,average_rvus);
    }
    println!();
}

fn build_coverage_tree_from_manifest(manifest:Manifest, source:&ProcessedSource) -> Result<CoverageMap, Box<dyn Error>> 
{
    println!("Building coverage tree.");
    let mut date_constraint_set: ConstraintSet<NaiveDateTime> = ConstraintSet::new();
    date_constraint_set.add(&is_not_holiday);

    let mut coverage_tree = CoverageMap::default();

    println!("Adding coverage.");
    coverage_tree.add_coverage_from_manifest(manifest)?;


    println!("Adding work to tree.");
    coverage_tree.add_work_from_source(&source, &date_constraint_set)?;

    Ok(coverage_tree)
}

const RVU_SUFFIX:&str="_rvu.csv";
const BVU_SUFFIX:&str="_bvu.csv";

impl MainCommon
{
    fn clear_coveragetree_analyses(
        coverage_audit_out:&str,
        coverage_audit_nowork_out:&str,
        coverage_analysis_out:&str
    )->()
    {
        let _=std::fs::remove_file(coverage_audit_out);
        let _=std::fs::remove_file(coverage_audit_nowork_out);
        let _=std::fs::remove_file(coverage_analysis_out.to_string()+RVU_SUFFIX);
        let _=std::fs::remove_file(coverage_analysis_out.to_string()+BVU_SUFFIX);
    }
    fn analyze_coveragetree(
        coverage_tree:&mut CoverageMap,
        coverage_audit_out:&str,
        coverage_audit_nowork_out:&str,
        coverage_analysis_out:&str
    ) -> Result<(), Box<dyn Error>> {
        
        let auditfile: File = File::create(coverage_audit_out)?;
        let mut writer = BufWriter::new(auditfile);
    
        let auditfile_nowork: File = File::create(coverage_audit_nowork_out)?;
        let mut writer_nowork = BufWriter::new(auditfile_nowork);
    
        let audit_result = audit(coverage_tree);
        audit_to_stream(&audit_result, &mut writer, &mut writer_nowork)?;

        let analysis = analyze_by_day_of_week(coverage_tree);
        analysis_to_csv(
            &analysis,
            coverage_analysis_out.to_owned() + "_rvu.csv",
            true,
        );
        analysis_to_csv(
            &analysis,
            coverage_analysis_out.to_owned() + "_bvu.csv",
            false,
        );

        Ok(())
    }

    pub fn analyze_rotations(
        &mut self,
    ) -> Result<(), Box<dyn Error>> {

        Self::clear_coveragetree_analyses(
            ACTIVE_COVERAGE_AUDIT_OUT,
            ACTIVE_COVERAGE_AUDIT_NOWORK_OUT,
            ACTIVE_COVERAGE_ANALYSIS_OUT
        );

        Self::analyze_coveragetree(
            &mut self.coverage_tree,
            ACTIVE_COVERAGE_AUDIT_OUT,
            ACTIVE_COVERAGE_AUDIT_NOWORK_OUT,
            ACTIVE_COVERAGE_ANALYSIS_OUT
        )?;

        Self::clear_coveragetree_analyses(
            PROPOSED_COVERAGE_AUDIT_OUT,
            PROPOSED_COVERAGE_AUDIT_NOWORK_OUT,
            PROPOSED_COVERAGE_ANALYSIS_OUT
        );

        match ManifestType::Proposed.get()
        {
            Ok(proposed_manifest) => {
                println!();
                println!("Analyzing proposed manifest.");
                let mut proposed_coverage_tree=build_coverage_tree_from_manifest(proposed_manifest, &self.source)?;
                
                Self::analyze_coveragetree(
                    &mut proposed_coverage_tree,
                    PROPOSED_COVERAGE_AUDIT_OUT,
                    PROPOSED_COVERAGE_AUDIT_NOWORK_OUT,
                    PROPOSED_COVERAGE_ANALYSIS_OUT
                )?;
            },
            Err(_) => () //nothing
        }

    
        Ok(())
    }

    fn volume_heatmap_to_json(coverage_tree:&CoverageMap, rotation_start:&NaiveDate, rotation_end:&NaiveDate, filename:String)-> Result<(), Box<dyn Error>>
    {
        let mut rotation_volume_heatmap = sort_volumes_by_rotation_date(coverage_tree);
        rotation_volume_heatmap.retain(
            |key,_value|
            {
                rotation_start<=key && rotation_end>=key
            }
        );
        
        analysis_to_plot(
            &mut rotation_volume_heatmap,
            filename,
        )
    }

    pub fn generate_frontend_statics(&mut self, facility_start:&NaiveDate, facility_end:&NaiveDate, rotation_start:&NaiveDate, rotation_end:&NaiveDate) -> Result<(), Box<dyn Error>> {
    
        let millistr = chrono::Local::now().timestamp_millis().to_string();
    
        use std::io::Write;
        
        let mut file = File::create("../frontend/src/commons/key.ts")?;
        writeln!(
            file,
            "//This file is automatically generated by the rust core! Please do not change."
        )?;
        writeln!(file, "export let key={}", millistr)?;
    
        std::fs::remove_dir_all(BASE)?;
        std::fs::create_dir(BASE)?;
    
        //Add volumes to the manifest before creating manifest json
        let mut manifest = ManifestType::Active.get()?;
        let mut mutable_temporary_coverage_tree=self.coverage_tree.clone();
        mutable_temporary_coverage_tree.populate_responsibility_volumes(&mut manifest, rotation_start, rotation_end)?;
        manifest.to_json(&(BASE.to_string() + "/active_rotation_manifest" + &millistr + ".json"))?;
        
        self.coverage_tree.to_json(&(BASE.to_string() + "/active_coverage_tree" + &millistr + ".json"))?;
    
        //Categories lists
        self.source.subspecialty_map
            .to_json(&(BASE.to_string() + "/exam_categories" + &millistr + ".json"))?;
    
        //Plots
        Self::volume_heatmap_to_json(&self.coverage_tree, rotation_start, rotation_end,BASE.to_string() + "/" + VOLUME_BY_DATE_ROTATION_ACTIVE + &millistr + ".json")?;
    
        let mut facility_volume_chart=sort_volumes_by_facility_and_date(&mut self.coverage_tree);
        facility_volume_chart.retain(
            |key,_value|
            {
                facility_start<=key && facility_end>=key
            }
        );
        volumes_by_facility_and_date_to_plot(
            &mut facility_volume_chart, 
            BASE.to_string() + "/" + VOLUME_BY_DATE_FACILITY + &millistr + ".json"
        )?;
    
        //Proposal
        match ManifestType::Proposed.get()
        {
            Ok(proposed_manifest) => {
                println!();
                println!("Generating proposal frontend statics.");
                let proposed_coverage_tree=build_coverage_tree_from_manifest(proposed_manifest, &self.source)?;
                Self::volume_heatmap_to_json(&proposed_coverage_tree, rotation_start, rotation_end, 
                    BASE.to_string() + "/" + VOLUME_BY_DATE_ROTATION_PROPOSED + &millistr + ".json"
                )?;

                let comparison=compare(&self.coverage_tree, &proposed_coverage_tree);
                comparison.to_json((BASE.to_string() + "/" + PROPOSED_DIFFERENTIAL + &millistr + ".json").as_str())?;
            },
            Err(_) => todo!(), //delete proposed files
        }

        Ok(())
    }
    
    pub fn perform_detailed_analysis(&mut self) -> Result<(), Box<dyn Error>> {
        self.detailed_analysis(chrono::Weekday::Mon, globals::MSK)?;
        self.detailed_analysis(chrono::Weekday::Sun, globals::MSK_WE_AH0C)?;
        Ok(())
    }
    
    fn detailed_analysis(
        &mut self,
        weekday: chrono::Weekday,
        rotation: &str,
    ) -> Result<(), Box<dyn Error>> {
        let details = details(&mut self.coverage_tree, weekday, rotation)?;
    
        println!("Detailed analysis for {}-{}", rotation, weekday);
        for (exam, count) in details.get_studies() {
            println!("{},{}", exam, count);
        }
    
        Ok(())
    }
}

const BASE: &str = "../frontend/static/data";

