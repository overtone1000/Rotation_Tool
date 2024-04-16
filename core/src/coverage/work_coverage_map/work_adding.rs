use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::collections::{hash_map::Entry, HashMap};

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime};

use crate::analysis::analysis_datum::WorkUnit;

use crate::coverage::coordinate::CoverageCoordinates;

use crate::coverage::distribution::get_normal_dist_weights;

use crate::globals::{map_SH_location_to_facility, siteid_to_sitename, SH_site_id, NON_RADIOLOGY, SH, WVH};
use crate::rotations::description::WrappedSortable;

use crate::error::source_error::SourceError;

use crate::source_data::processing::categorization::{
    build_salem_rvumap, check_categories_list,
};
use crate::source_data::processing::processed_source::ProcessedSource;
use crate::source_data::tables::exam_data::Exam;
use crate::source_data::tables::table::Table;
use crate::{
    constraints::ConstraintSet,
    dates::BUSINESS_DAYS_PER_YEAR,
    globals::{main_headers, tpc_headers, BUSINESS_DAYS, FACILITIES},
};

use super::generics::WorkCoverageMap;
use super::maps::CoverageMap;

fn get_SH_facility_from_metadata(exam:&Exam)->Option<String>{

    if exam.site_id!=SH_site_id {return None};

    let test=|facstr:&str|->Option<String>
    {
        if exam.accession.len()>=facstr.len() && (exam.accession[0..facstr.len()]).to_ascii_lowercase()
            == facstr.to_ascii_lowercase()
        {
            return Some(facstr.to_string());
        }
        None
    };

    //Check accession beginning against facility strings first
    for facility in FACILITIES {
        let testresult=test(facility);
        if(testresult.is_some()){return testresult;}
    }

    //Test ST and SV
    if test("ST").is_some() || test("SV").is_some(){return Some(SH.to_string());}

    //Then check against location
    map_SH_location_to_facility(&exam.location)
}

impl CoverageMap {
    pub fn add_work_from_source(
        &mut self,
        source: &ProcessedSource,
        date_constraints: &ConstraintSet<'_, NaiveDateTime>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _retval = CoverageMap::default();

        //let mut modality_map: HashMap<String, String> = HashMap::new();

        let exam_rvu_map = build_salem_rvumap(&source.main_data)?;
        
        let mut salem_weekday_count: HashMap<chrono::Weekday, u64> = HashMap::new();
        //Determine how many days worth for each weekday
        let mut dateset: HashSet<NaiveDate> = HashSet::new();
        for exam in source.main_data.iter()
        {
            if date_constraints.include(&exam.list_datetime) {
                dateset.insert(NaiveDate::from(exam.list_datetime));
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
        
        let mut excluded_by_reader:Vec<&Exam>=Vec::new();
        let mut excluded_as_nonradiology:Vec<&Exam>=Vec::new();

        //Process Data
        for exam in source.main_data.iter()
        {
            match source.readers.get(&exam.signer_acct_id)
            {
                Some(reader) => {
                    if reader.excluded
                    {
                        excluded_by_reader.push(&exam);
                    }
                    else
                    {
                        if date_constraints.include(&exam.list_datetime) {
                            let denominator = *salem_weekday_count
                                .get(&NaiveDate::from(exam.list_datetime).weekday())
                                .expect("All weekdays should be populated")
                                as f64;
            
                            //Build coords and populate maps with this row.
                            let coords: CoverageCoordinates = {
                                //Get subspecialty from exam code
                                let subspecialty = match source.subspecialty_map.get(&exam.exam_code) {
                                    Some(x) => x.to_string(),
                                    None => {
                                        return SourceError::generate_boxed(format!(
                                            "Invalid exam.procedure_code {} in exam_to_subspeciality_map",
                                            exam.exam_code
                                        ));
                                    }
                                };
            
                                //Try to determine facility from accession (good for separating SH, WB, WVH) and then location. If not valid, go by site ID. If not valid, go by location.
                                let mut selected_facility: Option<String> = None;
                                selected_facility=get_SH_facility_from_metadata(exam);
                                if selected_facility.is_none() {
                                    selected_facility=siteid_to_sitename(exam.site_id);
                                }
                                if selected_facility.is_none() {
                                    selected_facility = crate::globals::get_location_site_mapping(&exam.location);
                                }
                                let facility = match selected_facility {
                                    Some(x) => x,
                                    None => {
                                        return SourceError::generate_boxed(format!(
                                            "Could not determine facility for exam {:?}",exam,
                                        ));
                                    }
                                };
            
                                //Try context. If not valid, go by site map.
                                let context = match source.context_map.get(&exam.site_id) {
                                    Some(submap) => {
                                        match submap.get(&exam.location){
                                            Some(x) => x.to_string(),
                                            None => {
                                                return SourceError::generate_boxed(format!(
                                                    "Could not determine context {:?}",
                                                    exam
                                                ));
                                            }
                                        }
                                    },
                                    None => match crate::globals::get_location_site_mapping(&exam.location) {
                                        Some(x) => x,
                                        None => {
                                            return SourceError::generate_boxed(format!(
                                                "Could not determine context {:?}",
                                                exam
                                            ));
                                        }
                                    },
                                };
            
                                CoverageCoordinates {
                                    facility,
                                    subspecialty,
                                    context,
                                    //modality: modality.to_string(),
                                    weekday: exam.list_datetime.weekday(),
                                }
                            };
            
                            //Filter out non-radiology exams to make rotation front end simpler
                            let filtered=
                                coords.subspecialty == NON_RADIOLOGY ||
                                coords.context == NON_RADIOLOGY
                                ;
            
                            if filtered
                            {
                                excluded_as_nonradiology.push(exam)
                            }
                            else
                            {
                                let work: WorkUnit = {
                                    let rvu = match exam_rvu_map.get(&exam.exam_code) {
                                        Some(x) => x,
                                        None => {
                                            return SourceError::generate_boxed(format!(
                                                "Invalid exam.procedure_code {} in rvu map",
                                                exam.exam_code
                                            ));
                                        }
                                    };
            
                                    let bvu = match source.bvu_map.get(&exam.exam_code) {
                                        Some(x) => x,
                                        None => {
                                            return SourceError::generate_boxed(format!(
                                                "Invalid exam.procedure_code {} in bvu map",
                                                exam.exam_code
                                            ));
                                        }
                                    };
            
                                    WorkUnit::create(
                                        exam.list_datetime,
                                        *rvu,
                                        *bvu,
                                        denominator,
                                        exam.procedure_description.to_string()
                                    )
                                };
            
                                self.add_work(&coords, work);
                            }
                        }
                    }
                },
                None => (),
            }
        }

        let sum_rvus = |exams:&Vec<Exam>|->f64
        {
            let mut retval:f64=0.0;
            for exam in exams
            {
                match exam_rvu_map.get(&exam.exam_code) {
                    Some(x) => {
                        retval+=x;
                    },
                    None => {
                        eprintln!(
                            "Invalid exam.procedure_code {} in rvu map",
                            exam.exam_code
                        );
                    }
                }
            }
            retval
        };

        let sum_rvus_ref = |exams:Vec<&Exam>|->f64
        {
            let mut retval:f64=0.0;
            for exam in exams
            {
                match exam_rvu_map.get(&exam.exam_code) {
                    Some(x) => {
                        retval+=x;
                    },
                    None => {
                        eprintln!(
                            "Invalid exam.procedure_code {} in rvu map",
                            exam.exam_code
                        );
                    }
                }
            }
            retval
        };

        let mut rvu_total:f64=sum_rvus(&source.main_data);
        let mut rvus_excluded_by_reader:f64=sum_rvus_ref(excluded_by_reader);
        let mut rvus_filtered:f64=sum_rvus_ref(excluded_as_nonradiology);

        println!();
        println!("{}% of RVUs filtered by reader.",rvus_excluded_by_reader/rvu_total*100.0);
        println!("{}% of RVUs filtered by non-radiology categorization.",rvus_filtered/rvu_total*100.0);
        println!();

        Ok(())
    }
}
