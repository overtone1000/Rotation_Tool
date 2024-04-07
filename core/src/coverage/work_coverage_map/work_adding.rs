use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};

use crate::analysis::analysis_datum::WorkUnit;

use crate::coverage::coordinate::CoverageCoordinates;

use crate::coverage::distribution::get_normal_dist_weights;

use crate::globals::siteid_to_sitename;
use crate::rotations::description::WrappedSortable;

use crate::error::source_error::SourceError;

use crate::source_data::processing::categorization::{
    build_salem_bvumap, build_salem_rvumap, check_categories_list,
};
use crate::source_data::processing::processed_source::ProcessedSource;
use crate::source_data::tables::table::Table;
use crate::{
    constraints::ConstraintSet,
    dates::BUSINESS_DAYS_PER_YEAR,
    globals::{main_headers, tpc_headers, BUSINESS_DAYS, SITES},
};

use super::generics::WorkCoverageMap;
use super::maps::CoverageMap;

impl CoverageMap {
    pub fn add_work_from_source(
        &mut self,
        source: &ProcessedSource,
        date_constraints: &ConstraintSet<'_, NaiveDateTime>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _retval = CoverageMap::default();

        //let mut modality_map: HashMap<String, String> = HashMap::new();

        let exam_rvu_map = build_salem_rvumap(&source.main_data_table)?;
        let exam_bvu_map: HashMap<String, f64> = build_salem_bvumap(&source.bvu_data_table)?;

        let exam_code_map = source.exam_categories_list;

        let mut salem_weekday_count: HashMap<chrono::Weekday, u64> = HashMap::new();
        //Determine how many days worth for each weekday
        let mut dateset: HashSet<NaiveDate> = HashSet::new();
        source.main_data_table.for_each(
            |exam| {
                if date_constraints.include(&exam.list_datetime) {
                    dateset.insert(NaiveDate::from(exam.list_datetime));
                }
                Ok(())
            }
        );

        for date in dateset {
            match salem_weekday_count.entry(date.weekday()) {
                Entry::Occupied(x) => {
                    let mutable = x.into_mut();
                    *mutable += 1;
                }
                Entry::Vacant(x) => {
                    x.insert(1);
                }
            };
        }
        
        //Process Salem Data
        source.main_data_table.for_each(
            |exam| {
                if date_constraints.include(&exam.list_datetime) {
                    let denominator = *salem_weekday_count
                        .get(&NaiveDate::from(exam.list_datetime).weekday())
                        .expect("All weekdays should be populated")
                        as f64;

                    //Build coords and populate maps with this row.
                    let coords: CoverageCoordinates = {
                        //Get subspecialty from exam code
                        let subspecialty = match source.exam_to_subspecialty_map.get(&exam.procedure_code) {
                            Some(x) => x.to_string(),
                            None => {
                                return SourceError::generate_boxed(format!(
                                    "Invalid exam.procedure_code {} in exam_to_subspeciality_map",
                                    exam.procedure_code
                                ));
                            }
                        };

                        //Try to determine site from accession (good for separating SH, WB, WVH). If not valid, go by site ID. If not valid, go by location.
                        let mut selected_site: Option<String> = None;
;
                        for site in SITES {
                            if (exam.accession[0..site.len()]).to_ascii_uppercase()
                                == site.to_string().to_ascii_uppercase()
                            {
                                selected_site = Some(site.to_string());
                                break;
                            }
                        }
                        if selected_site.is_none() {
                            selected_site=siteid_to_sitename(&exam.site_id);
                        }
                        if selected_site.is_none() {
                            selected_site = crate::globals::get_location_site_mapping(&exam.location);
                        }
                        let site = match selected_site {
                            Some(x) => x,
                            None => {
                                return SourceError::generate_boxed(format!(
                                    "Could not determine site for exam {:?}",exam,
                                ));
                            }
                        };

                        //Try context. If not valid, go by site map.
                        let context = match source.site_and_location_to_context_map.get(&exam.site_id) {
                            Some(x) => {
                                match x.get(&exam.location){
                                    Some(x) => x.to_string(),
                                    None => {
                                        return SourceError::generate_boxed(format!(
                                            "Could not determine context for location {}",
                                            exam.location
                                        ));
                                    }
                                }
                            },
                            None => match crate::globals::get_location_site_mapping(&exam.location) {
                                Some(x) => x,
                                None => {
                                    return SourceError::generate_boxed(format!(
                                        "Could not determine context for location {}",
                                        exam.location
                                    ));
                                }
                            },
                        };

                        CoverageCoordinates {
                            site,
                            subspecialty,
                            context,
                            //modality: modality.to_string(),
                            weekday: exam.list_datetime.weekday(),
                        }
                    };

                    let work: WorkUnit = {
                        let rvu = match exam_rvu_map.get(&exam.procedure_code) {
                            Some(x) => x,
                            None => {
                                return SourceError::generate_boxed(format!(
                                    "Invalid exam.procedure_code {} in exam_to_subspeciality_map",
                                    exam.procedure_code
                                ));
                            }
                        };

                        let bvu = match exam_bvu_map.get(&exam.procedure_code) {
                            Some(x) => x,
                            None => {
                                return SourceError::generate_boxed(format!(
                                    "Invalid exam.procedure_code {} in exam_to_subspeciality_map",
                                    exam.procedure_code
                                ));
                            }
                        };

                        WorkUnit::create(
                            exam.list_datetime,
                            *rvu,
                            *bvu,
                            denominator,
                            exam_code_map
                                .get(&exam.procedure_code)
                                .expect("Should be there!")
                                .exam
                                .to_string(),
                        )
                    };

                    self.add_work(&coords, work);
                }
                Ok(())
            }
        );
        
        /*
        //Add TPC, which doesn't go by number of dates
        let distribution_weights = get_normal_dist_weights();
        for row_i in source.tpc_data_table.row_indices() {
            let exam.procedure_code = source
                .tpc_data_table
                .get_val(&tpc_headers::PertinentHeaders::ExamCode.get_label(), &row_i)?;

            let number_str = source.tpc_data_table.get_val(
                &tpc_headers::PertinentHeaders::NumberIn2022.get_label(),
                &row_i,
            )?;

            let number_of_exams = match number_str.parse::<f64>() {
                Ok(val) => val,
                Err(e) => {
                    return SourceError::generate_boxed(format!("{:?}", e));
                }
            };

            let rvus_per_exam = match exam_rvu_map.get(&exam.procedure_code) {
                Some(val) => val.to_owned(),
                None => {
                    return SourceError::generate_boxed(format!("Bad exam code {}", exam.procedure_code));
                }
            };
            let bvus_per_exam = match exam_bvu_map.get(&exam.procedure_code) {
                Some(val) => val.to_owned(),
                None => {
                    return SourceError::generate_boxed(format!("Bad exam code {}", exam.procedure_code));
                }
            };

            let subspecialty = match source.exam_to_subspecialty_map.get(&exam.procedure_code) {
                None => {
                    return SourceError::generate_boxed(format!("Bad exam code {}", exam.procedure_code));
                }
                Some(val) => val.to_owned(),
            };

            /*
            let modality = match modality_map.get(&exam.procedure_code) {
                None => {
                    return SourceError::generate_boxed(format!("Bad exam code {}", exam.procedure_code));
                }
                Some(val) => val.to_owned(),
            };
            */

            for weekday in BUSINESS_DAYS {
                let coords = CoverageCoordinates {
                    site: crate::globals::TPC.to_string(),
                    context: crate::globals::OUTPATIENT.to_string(),
                    //modality: modality.to_string(),
                    subspecialty: subspecialty.to_string(),
                    weekday: **weekday,
                };

                let mut date = NaiveDate::default();
                date = date + Duration::days(**weekday as i64 - date.weekday() as i64);

                println!("THIS INTRODUCES BAD DATES!");

                if date.weekday() != **weekday {
                    return SourceError::generate_boxed("Weekday math is wrong.".to_string());
                }

                for key in distribution_weights.keys() {
                    let work = WorkUnit::create(
                        NaiveDateTime::new(date, *key),
                        number_of_exams
                            * rvus_per_exam
                            * (*distribution_weights.get(key).expect("Expected")) as f64,
                        number_of_exams
                            * bvus_per_exam
                            * (*distribution_weights.get(key).expect("Expected")) as f64,
                        BUSINESS_DAYS_PER_YEAR,
                        exam.procedure_code_map
                            .get(&exam.procedure_code)
                            .expect("Should be there!")
                            .exam
                            .to_string(),
                    );
                    self.add_work(&coords, work);
                }
            }
        }
         */

        Ok(())
    }
}
