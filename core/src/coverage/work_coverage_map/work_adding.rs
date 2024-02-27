
use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};





use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};


use crate::analysis::analysis_datum::WorkUnit;

use crate::coverage::coordinate::CoverageCoordinates;

use crate::coverage::distribution::get_normal_dist_weights;






use crate::rotations::description::WrappedSortable;



use crate::error::source_error::SourceError;


use crate::source_data::processing::categorization::{
    build_salem_bvumap, build_salem_rvumap, get_categories_map,
};
use crate::source_data::processing::processed_source::ProcessedSource;
use crate::{
    constraints::ConstraintSet,
    dates::BUSINESS_DAYS_PER_YEAR,
    globals::{main_headers, tpc_headers, BUSINESS_DAYS, SITES},
};

use super::generics::{WorkCoverageMap};
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

        let exam_code_map = get_categories_map(source)?;

        let mut salem_weekday_count: HashMap<chrono::Weekday, u64> = HashMap::new();
        //Determine how many days worth for each weekday
        {
            let mut dateset: HashSet<NaiveDate> = HashSet::new();
            for row_i in source.main_data_table.row_indices() {
                let datetimestring = source.main_data_table.get_val(
                    &main_headers::PertinentHeaders::ScheduledDatetime.get_label(),
                    &row_i,
                )?;

                let datetime =
                    match NaiveDateTime::parse_from_str(&datetimestring, "%m/%d/%y %H:%M") {
                        Ok(x) => x,
                        Err(x) => {
                            return Err(Box::new(x));
                        }
                    };

                if date_constraints.include(&datetime) {
                    dateset.insert(NaiveDate::from(datetime));
                }
            }

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
        }
        //Process Salem Data
        for row_i in source.main_data_table.row_indices() {
            let datetimestring = source.main_data_table.get_val(
                &main_headers::PertinentHeaders::ScheduledDatetime.get_label(),
                &row_i,
            )?;

            let datetime = match NaiveDateTime::parse_from_str(&datetimestring, "%m/%d/%y %H:%M") {
                Ok(x) => x,
                Err(x) => {
                    return Err(Box::new(x));
                }
            };

            if date_constraints.include(&datetime) {
                let denominator = *salem_weekday_count
                    .get(&NaiveDate::from(datetime).weekday())
                    .expect("All weekdays should be populated")
                    as f64;

                let location = source.main_data_table.get_val(
                    &main_headers::PertinentHeaders::Location.get_label(),
                    &row_i,
                )?;
                let exam_code = source.main_data_table.get_val(
                    &main_headers::PertinentHeaders::ProcedureCode.get_label(),
                    &row_i,
                )?;

                //Build coords and populate maps with this row.
                let coords: CoverageCoordinates = {
                    //Get subspecialty from exam code
                    let subspecialty = match source.exam_to_subspecialty_map.get(&exam_code) {
                        Some(x) => x.to_string(),
                        None => {
                            return SourceError::generate_boxed(format!(
                                "Invalid exam_code {} in exam_to_subspeciality_map",
                                exam_code
                            ));
                        }
                    };

                    //Try site. If not valid, go by location.
                    let mut selected_site: Option<String> = None;
                    let listed_site = source.main_data_table.get_val(
                        &main_headers::PertinentHeaders::Accession.get_label(),
                        &row_i,
                    )?;
                    for site in SITES {
                        if (listed_site[0..site.len()]).to_ascii_uppercase()
                            == site.to_string().to_ascii_uppercase()
                        {
                            selected_site = Some(site.to_string());
                            break;
                        }
                    }
                    if selected_site.is_none() {
                        selected_site = crate::globals::get_location_site_mapping(&location);
                    }
                    let site = match selected_site {
                        Some(x) => x,
                        None => {
                            return SourceError::generate_boxed(format!(
                                "Could not determine site for row {}",
                                row_i
                            ));
                        }
                    };

                    //Try context. If not valid, go by site map.
                    let context = match source.location_to_context_map.get(&location) {
                        Some(x) => x.to_string(),
                        None => match crate::globals::get_location_site_mapping(&location) {
                            Some(x) => x,
                            None => {
                                return SourceError::generate_boxed(format!(
                                    "Could not determine context for location {}",
                                    location
                                ));
                            }
                        },
                    };

                    //Get modality, but check for aliases
                    /*
                    let listed_modality = source.main_data_table.get_val(
                        &main_headers::PertinentHeaders::Modality.get_label(),
                        &row_i,
                    )?;
                    let mut selected_modality: Option<String> = None;
                    for modality in MODALITIES {
                        if *modality == listed_modality {
                            selected_modality = Some(modality.to_string());
                            break;
                        }
                    }
                    match selected_modality {
                        None => {
                            selected_modality = crate::globals::get_modality_alias(&listed_modality);
                        }
                        _ => {}
                    }
                    match selected_modality {
                        None => {
                            selected_modality = crate::globals::get_modality_from_procedure_desc(
                                source.main_data_table.get_val(
                                    &main_headers::PertinentHeaders::Exam.get_label(),
                                    &row_i,
                                )?,
                            )
                        }
                        _ => {}
                    }
                    let modality = match selected_modality {
                        Some(x) => x,
                        None => {
                            return SourceError::generate_boxed(format!(
                                "Could not determine modality for row {}",
                                row_i
                            ));
                        }
                    };
                    if !modality_map.contains_key(&exam_code) {
                        modality_map.insert(exam_code.to_owned(), modality.to_string());
                    }
                    */

                    CoverageCoordinates {
                        site,
                        subspecialty,
                        context,
                        //modality: modality.to_string(),
                        weekday: datetime.weekday(),
                    }
                };

                let work: WorkUnit = {
                    let rvu = match exam_rvu_map.get(&exam_code) {
                        Some(x) => x,
                        None => {
                            return SourceError::generate_boxed(format!(
                                "Invalid exam_code {} in exam_to_subspeciality_map",
                                exam_code
                            ));
                        }
                    };

                    let bvu = match exam_bvu_map.get(&exam_code) {
                        Some(x) => x,
                        None => {
                            return SourceError::generate_boxed(format!(
                                "Invalid exam_code {} in exam_to_subspeciality_map",
                                exam_code
                            ));
                        }
                    };

                    WorkUnit::create(
                        datetime,
                        *rvu,
                        *bvu,
                        denominator,
                        exam_code_map
                            .get(&exam_code)
                            .expect("Should be there!")
                            .exam
                            .to_string(),
                    )
                };

                self.add_work(&coords, work);
            }
        }
        //Add TPC, which doesn't go by number of dates
        let distribution_weights = get_normal_dist_weights();
        for row_i in source.tpc_data_table.row_indices() {
            let exam_code = source
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

            let rvus_per_exam = match exam_rvu_map.get(&exam_code) {
                Some(val) => val.to_owned(),
                None => {
                    return SourceError::generate_boxed(format!("Bad exam code {}", exam_code));
                }
            };
            let bvus_per_exam = match exam_bvu_map.get(&exam_code) {
                Some(val) => val.to_owned(),
                None => {
                    return SourceError::generate_boxed(format!("Bad exam code {}", exam_code));
                }
            };

            let subspecialty = match source.exam_to_subspecialty_map.get(&exam_code) {
                None => {
                    return SourceError::generate_boxed(format!("Bad exam code {}", exam_code));
                }
                Some(val) => val.to_owned(),
            };

            /*
            let modality = match modality_map.get(&exam_code) {
                None => {
                    return SourceError::generate_boxed(format!("Bad exam code {}", exam_code));
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
                        exam_code_map
                            .get(&exam_code)
                            .expect("Should be there!")
                            .exam
                            .to_string(),
                    );
                    self.add_work(&coords, work);
                }
            }
        }

        Ok(())
    }
}