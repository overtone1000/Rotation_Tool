use std::collections::HashSet;
use std::collections::{hash_map::Entry, HashMap};

use std::fmt::Debug;

use std::hash::Hash;
use std::io::Write;
use std::ops::AddAssign;
use std::str::FromStr;

use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, Timelike};

use crate::globals::{self, ALL_DAYS};
use crate::rotations::manifest::Manifest;
use crate::rotations::rotation_error::RotationManifestParseError;
use crate::rotations::time_modifiers::{NEXT_MIDNIGHT, THIS_MIDNIGHT, TimeSinceMidnight};
use crate::rotations::timespan::parse_time_span;
use crate::{
    categorization::{build_salem_bvumap, build_salem_rvumap},
    constraints::ConstraintSet,
    dates::BUSINESS_DAYS_PER_YEAR,
    globals::{main_headers, tpc_headers, BUSINESS_DAYS, MODALITIES, SITES},
    processed_source::ProcessedSource,
};

use super::fractional_coverage::FractionalCoverageUnit;
use super::source_error::SourceError;
use super::temporal_coverage::{weekday_plus, TemporalCoverageUnit};

#[derive(Eq, Hash, PartialEq, Clone)]
pub struct CoverageCoordinates {
    site: String,
    subspecialty: String,
    context: String,
    modality: String,
    weekday: chrono::Weekday,
}

impl PartialOrd for CoverageCoordinates {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.site.partial_cmp(&other.site) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.subspecialty.partial_cmp(&other.subspecialty) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.context.partial_cmp(&other.context) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.modality.partial_cmp(&other.modality) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.weekday
            .num_days_from_monday()
            .partial_cmp(&other.weekday.num_days_from_monday())
    }
}

impl Ord for CoverageCoordinates {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(x) => x,
            None => std::cmp::Ordering::Equal,
        }
    }
}

impl Default for CoverageCoordinates {
    fn default() -> Self {
        Self {
            site: Default::default(),
            subspecialty: Default::default(),
            context: Default::default(),
            modality: Default::default(),
            weekday: chrono::Weekday::Sun,
        }
    }
}

#[derive(Default, Debug)]
pub struct WorkUnit {
    datetime: NaiveDateTime,
    rvu: f64,
    bvu: f64,
}

#[derive(Clone)]
pub enum CoverageUnit {
    Temporal(TemporalCoverageUnit),
    WeekFraction(FractionalCoverageUnit),
}

pub trait WorkCollector {
    fn collect_work(&self, workday: &CoverageAndWorkDay) -> AnalysisDatum;
}

impl WorkCollector for CoverageUnit {
    fn collect_work(&self, workday: &CoverageAndWorkDay) -> AnalysisDatum {
        let _retval: AnalysisDatum = AnalysisDatum {
            total_rvu: 0.0,
            total_bvu: 0.0,
        };

        match self {
            CoverageUnit::Temporal(s) => s.collect_work(workday),
            CoverageUnit::WeekFraction(s) => s.collect_work(workday),
        }
    }
}

#[derive(Debug)]
pub enum Coverage {
    Temporal(Vec<TemporalCoverageUnit>),
    Fractional(Vec<FractionalCoverageUnit>),
}

impl Coverage {
    pub fn add(&mut self, coverage: CoverageUnit) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Coverage::Temporal(coverages) => match coverage {
                CoverageUnit::Temporal(new_coverage) => {
                    coverages.push(new_coverage);
                }
                CoverageUnit::WeekFraction(_new_coverage) => {
                    return SourceError::generate_boxed(
                        "Mixing fractional and temporal coverage types is not allowed.".to_string(),
                    );
                }
            },
            Coverage::Fractional(coverages) => match coverage {
                CoverageUnit::Temporal(_new_coverage) => {
                    return SourceError::generate_boxed(
                        "Mixing fractional and temporal coverage types is not allowed.".to_string(),
                    );
                }
                CoverageUnit::WeekFraction(new_coverage) => {
                    coverages.push(new_coverage);
                }
            },
        }
        Ok(())
    }
}

#[derive(Debug)]
#[derive(Default)]
pub struct CoverageAndWorkDay {
    pub coverages: Option<Coverage>,
    pub work: Vec<WorkUnit>,
}

impl CoverageAndWorkDay {
    pub fn add_work(&mut self, work: WorkUnit) {
        self.work.push(work)
    }
    pub fn add_coverage(
        &mut self,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match &mut self.coverages {
            Some(x) => x.add(coverage),
            None => {
                let mut instantiated_coverages: Coverage = match coverage {
                    CoverageUnit::Temporal(_) => Coverage::Temporal(Vec::new()),
                    CoverageUnit::WeekFraction(_) => Coverage::Fractional(Vec::new()),
                };
                let retval = instantiated_coverages.add(coverage);
                self.coverages = Some(instantiated_coverages);
                retval
            }
        }
    }
}



#[derive(Default, Debug)]
pub struct MalformedCoverage {
    gaps: Vec<(TimeSinceMidnight, TimeSinceMidnight, String, f64)>,
    overlaps: Vec<String>,
    incorrect_fraction: Option<f64>,
}

pub enum CoverageError {
    NoWork,
    NoCoverage(f64),
    MalformedCoverage(MalformedCoverage),
}

impl CoverageAndWorkDay {
    fn sort_coverage(&mut self) {
        match &mut self.coverages {
            Some(coverages) => match coverages {
                Coverage::Temporal(temporal_coverages) => {
                    temporal_coverages.sort();
                }
                Coverage::Fractional(_) => (),
            },
            None => (),
        }
    }

    pub fn get_work_in_timespan(
        &self,
        start: TimeSinceMidnight,
        end: TimeSinceMidnight,
    ) -> AnalysisDatum {
        let mut retval: AnalysisDatum = AnalysisDatum {
            total_rvu: 0.0,
            total_bvu: 0.0,
        };
        for work in &self.work {
            let tsm = TimeSinceMidnight::from_minutes(
                (work.datetime.num_seconds_from_midnight() / 60).into(),
            );
            if start <= tsm && tsm < end {
                retval.add_workunit(work);
            }
        }
        retval
    }

    fn audit_coverage(&mut self) -> CoverageError {
        if self.work.is_empty() {
            return CoverageError::NoWork;
        }

        self.sort_coverage();

        match &self.coverages {
            None => {
                CoverageError::NoCoverage(
                    self.get_work_in_timespan(THIS_MIDNIGHT, NEXT_MIDNIGHT)
                        .total_rvu,
                )
            }
            Some(coverages) => {
                let mut retval = MalformedCoverage::default();
                match coverages {
                    Coverage::Temporal(temporal_coverages) => {
                        match temporal_coverages.split_first() {
                            Some((mut farthest_unit, rest)) => {
                                //Check from midnight
                                if farthest_unit.starts_after_this_midnight() {
                                    let rvus = &self
                                        .get_work_in_timespan(THIS_MIDNIGHT, farthest_unit.start);
                                    retval.gaps.push((
                                        THIS_MIDNIGHT,
                                        farthest_unit.start,
                                        farthest_unit.to_string() + " starts after midnight",
                                        rvus.total_rvu,
                                    ))
                                }

                                for cu in rest {
                                    if farthest_unit.end_overlaps_other(cu)
                                    //Check overlap
                                    {
                                        retval.overlaps.push(
                                            TemporalCoverageUnit::get_overlap_desc(
                                                farthest_unit,
                                                cu,
                                            ),
                                        );
                                    } else if farthest_unit.gap_between_end_and_other(cu)
                                    //Check gap
                                    {
                                        let rvus =
                                            &self.get_work_in_timespan(farthest_unit.end, cu.start);
                                        retval.gaps.push((
                                            farthest_unit.end,
                                            cu.start,
                                            TemporalCoverageUnit::get_overlap_desc(
                                                farthest_unit,
                                                cu,
                                            ),
                                            rvus.total_rvu,
                                        ));
                                    }

                                    //Adjust prior_end
                                    if cu.ends_after_other(farthest_unit) {
                                        farthest_unit = cu;
                                    }
                                }
                                //Check through midnight
                                if farthest_unit.ends_before_next_midnight() {
                                    let rvus = &self
                                        .get_work_in_timespan(farthest_unit.end, NEXT_MIDNIGHT);
                                    retval.gaps.push((
                                        farthest_unit.end,
                                        NEXT_MIDNIGHT,
                                        farthest_unit.to_string() + " ends before midnight",
                                        rvus.total_rvu,
                                    ));
                                }
                            }
                            None => (),
                        };
                    }
                    Coverage::Fractional(fractional_coverages) => {
                        let mut sum: f64 = 0.0;
                        for coverage in fractional_coverages {
                            sum += coverage.get_fraction();
                        }

                        if (sum - 1.0).abs() > 0.001 {
                            retval.incorrect_fraction = Some(sum);
                        }
                    }
                }
                CoverageError::MalformedCoverage(retval)
            }
        }
    }
}

pub trait WorkCoverageMap {
    fn add_work(&mut self, coords: &CoverageCoordinates, work: WorkUnit);
    fn add_coverage(
        &mut self,
        coords: &CoverageCoordinates,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
pub trait CoordinateMap<'a, T, U>
where
    T: 'a + Debug + Eq + PartialEq + Hash,
    U: Default + Debug,
{
    fn get_map(&mut self) -> &mut HashMap<T, U>;
    fn get_coordinate(coords: &CoverageCoordinates) -> T;
    fn get_branch(&'a mut self, coords: &'a CoverageCoordinates) -> &mut U {
        let key = Self::get_coordinate(coords);
        let retval = match (*self.get_map()).entry(key) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(U::default()),
        };
        retval
    }
}

#[derive(Default, Debug)]
pub struct WeekdayMap {
    map: HashMap<chrono::Weekday, CoverageAndWorkDay>,
}
impl WeekdayMap {
    fn default() -> WeekdayMap {
        let mut map: HashMap<chrono::Weekday, CoverageAndWorkDay> = HashMap::new();
        map.insert(chrono::Weekday::Mon, CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Tue, CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Wed, CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Thu, CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Fri, CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Sat, CoverageAndWorkDay::default());
        map.insert(chrono::Weekday::Sun, CoverageAndWorkDay::default());
        WeekdayMap { map }
    }
}
impl WorkCoverageMap for WeekdayMap {
    fn add_work(&mut self, coords: &CoverageCoordinates, work: WorkUnit) {
        self.get_branch(coords).add_work(work);
    }
    fn add_coverage(
        &mut self,
        coords: &CoverageCoordinates,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match &coverage {
            CoverageUnit::Temporal(_) => self.get_branch(coords).add_coverage(coverage),
            CoverageUnit::WeekFraction(_) => {
                for weekday in ALL_DAYS {
                    let mut pseudocoords = coords.clone();
                    pseudocoords.weekday = **weekday;
                    match self
                        .get_branch(&pseudocoords)
                        .add_coverage(coverage.to_owned())
                    {
                        Ok(_) => (),
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Ok(())
            }
        }
    }
}
impl<'a> CoordinateMap<'a, chrono::Weekday, CoverageAndWorkDay> for WeekdayMap {
    fn get_map(&mut self) -> &mut HashMap<chrono::Weekday, CoverageAndWorkDay> {
        &mut self.map
    }
    fn get_coordinate(coords: &CoverageCoordinates) -> chrono::Weekday {
        coords.weekday
    }
}

#[derive(Default, Debug)]
pub struct ModalityMap {
    map: HashMap<String, WeekdayMap>,
}
impl<'a> CoordinateMap<'a, String, WeekdayMap> for ModalityMap {
    fn get_map(&mut self) -> &mut HashMap<String, WeekdayMap> {
        &mut self.map
    }
    fn get_coordinate(coords: &CoverageCoordinates) -> String {
        coords.modality.clone()
    }
}
impl WorkCoverageMap for ModalityMap {
    fn add_work(&mut self, coords: &CoverageCoordinates, work: WorkUnit) {
        self.get_branch(coords).add_work(coords, work)
    }

    fn add_coverage(
        &mut self,
        coords: &CoverageCoordinates,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.get_branch(coords).add_coverage(coords, coverage)
    }
}

#[derive(Default, Debug)]
pub struct ContextMap {
    map: HashMap<String, ModalityMap>,
}
impl<'a> CoordinateMap<'a, String, ModalityMap> for ContextMap {
    fn get_map(&mut self) -> &mut HashMap<String, ModalityMap> {
        &mut self.map
    }
    fn get_coordinate(coords: &CoverageCoordinates) -> String {
        coords.context.clone()
    }
}
impl WorkCoverageMap for ContextMap {
    fn add_work(&mut self, coords: &CoverageCoordinates, work: WorkUnit) {
        self.get_branch(coords).add_work(coords, work)
    }

    fn add_coverage(
        &mut self,
        coords: &CoverageCoordinates,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.get_branch(coords).add_coverage(coords, coverage)
    }
}

#[derive(Default, Debug)]
pub struct SubspecialtyMap {
    map: HashMap<String, ContextMap>,
}
impl<'a> CoordinateMap<'a, String, ContextMap> for SubspecialtyMap {
    fn get_map(&mut self) -> &mut HashMap<String, ContextMap> {
        &mut self.map
    }
    fn get_coordinate(coords: &CoverageCoordinates) -> String {
        coords.subspecialty.clone()
    }
}
impl WorkCoverageMap for SubspecialtyMap {
    fn add_work(&mut self, coords: &CoverageCoordinates, work: WorkUnit) {
        self.get_branch(coords).add_work(coords, work)
    }

    fn add_coverage(
        &mut self,
        coords: &CoverageCoordinates,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.get_branch(coords).add_coverage(coords, coverage)
    }
}

fn testcoords() -> CoverageCoordinates {
    CoverageCoordinates {
        site: "SH".to_string(),
        subspecialty: "Diagnostic Mamm".to_string(),
        context: "ED".to_string(),
        modality: "US".to_string(),
        weekday: chrono::Weekday::Mon,
    }
}

pub struct AnalysisDatum {
    pub total_rvu: f64,
    pub total_bvu: f64,
}

impl AddAssign for AnalysisDatum {
    fn add_assign(&mut self, rhs: Self) {
        self.total_rvu += rhs.total_rvu;
        self.total_bvu += rhs.total_bvu;
    }
}

impl AnalysisDatum {
    pub fn scale(&mut self, scale: f64) {
        self.total_rvu *= scale;
        self.total_bvu *= scale;
    }

    pub fn add_workunit(&mut self, rhs: &WorkUnit) {
        self.total_rvu += rhs.rvu;
        self.total_bvu += rhs.bvu;
    }
}

#[derive(Default)]
pub struct CoverageMap {
    map: HashMap<String, SubspecialtyMap>,
}
impl CoverageMap {
    pub fn add_work_from_source(
        &mut self,
        source: ProcessedSource,
        date_constraints: &ConstraintSet<'_, NaiveDateTime>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _retval = CoverageMap::default();

        let mut modality_map: HashMap<String, String> = HashMap::new();

        let exam_rvu_map = build_salem_rvumap(&source.main_data_table)?;
        let exam_bvu_map: HashMap<String, f64> = build_salem_bvumap(&source.bvu_data_table)?;

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

                    CoverageCoordinates {
                        site,
                        subspecialty,
                        context,
                        modality: modality.to_string(),
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

                    WorkUnit {
                        datetime,
                        rvu: *rvu / denominator, //divide by denominator to account for aggregation of many days of data
                        bvu: *bvu / denominator, //divide by denominator to account for aggregation of many days of data
                    }
                };

                self.add_work(&coords, work);
            }
        }
        //Add TPC, which doesn't go by number of dates
        let weights = crate::time::get_normal_dist_weights();
        for row_i in source.tpc_data_table.row_indices() {
            let exam_code = source.tpc_data_table.get_val(
                &tpc_headers::PertinentHeaders::ExamCode.get_label(),
                &row_i,
            )?;
            let number_str = source.tpc_data_table.get_val(
                &tpc_headers::PertinentHeaders::NumberIn2022.get_label(),
                &row_i,
            )?;

            let number = match number_str.parse::<f64>() {
                Ok(val) => val,
                Err(e) => {
                    return SourceError::generate_boxed(format!("{:?}", e));
                }
            };

            let number_per_business_day = number / BUSINESS_DAYS_PER_YEAR;

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

            let rvus_per_business_day = number_per_business_day * rvus_per_exam;
            let bvus_per_business_day = number_per_business_day * bvus_per_exam;

            let subspecialty = match source.exam_to_subspecialty_map.get(&exam_code) {
                None => {
                    return SourceError::generate_boxed(format!("Bad exam code {}", exam_code));
                }
                Some(val) => val.to_owned(),
            };

            let modality = match modality_map.get(&exam_code) {
                None => {
                    return SourceError::generate_boxed(format!("Bad exam code {}", exam_code));
                }
                Some(val) => val.to_owned(),
            };

            for weekday in BUSINESS_DAYS {
                let coords = CoverageCoordinates {
                    site: crate::globals::TPC.to_string(),
                    context: crate::globals::OUTPATIENT.to_string(),
                    modality: modality.to_string(),
                    subspecialty: subspecialty.to_string(),
                    weekday: **weekday,
                };

                let mut date = NaiveDate::default();
                date = date + Duration::days(**weekday as i64 - date.weekday() as i64);

                if date.weekday() != **weekday {
                    return SourceError::generate_boxed("Weekday math is wrong.".to_string());
                }

                for key in weights.keys() {
                    let work = WorkUnit {
                        datetime: NaiveDateTime::new(date, *key),
                        rvu: rvus_per_business_day * (*weights.get(key).expect("Expected")) as f64,
                        bvu: bvus_per_business_day * (*weights.get(key).expect("Expected")) as f64,
                    };
                    self.add_work(&coords, work);
                }
            }
        }

        Ok(())
    }

    pub fn add_coverage_from_manifest(
        &mut self,
        manifest: Manifest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let all_weekdays_strings: &[&str; 7] = &[
            &chrono::Weekday::Mon.to_string(),
            &chrono::Weekday::Tue.to_string(),
            &chrono::Weekday::Wed.to_string(),
            &chrono::Weekday::Thu.to_string(),
            &chrono::Weekday::Fri.to_string(),
            &chrono::Weekday::Sat.to_string(),
            &chrono::Weekday::Sun.to_string(),
        ];

        let mut coords: CoverageCoordinates = CoverageCoordinates::default();
        let _testcoords = testcoords();

        for rotation_description in &manifest.rotation_manifest {
            match &rotation_description.responsibilities {
                Some(responsibilities) => {
                    for responsibility in responsibilities {
                        for site in responsibility.sites.to_vec(globals::SITES) {
                            coords.site = site.to_string();
                            for subspecialty in responsibility
                                .subspecialties
                                .to_vec(globals::SUBSPECIALTIES)
                            {
                                coords.subspecialty = subspecialty.to_string();
                                for context in responsibility.contexts.to_vec(globals::CONTEXTS) {
                                    coords.context = context.to_string();
                                    for modality in
                                        responsibility.modalities.to_vec(globals::MODALITIES)
                                    {
                                        coords.modality = modality.to_string();
                                        for weekday_string in
                                            responsibility.days.to_vec(all_weekdays_strings)
                                        {
                                            let weekday = match chrono::Weekday::from_str(&weekday_string){
                                            Ok(x) => x,
                                            Err(_) => return RotationManifestParseError::generate_boxed(0,"".to_string()),
                                        };

                                            if responsibility.time_periods.is_some()
                                                && responsibility.weekly_fraction.is_some()
                                            {
                                                return RotationManifestParseError::generate_boxed(0,"'time_periods' and 'fraction' have both been provided. One and only one must be provided.".to_string());
                                            }
                                            if responsibility.time_periods.is_none()
                                                && responsibility.weekly_fraction.is_none()
                                            {
                                                return RotationManifestParseError::generate_boxed(0,"Neither 'time_periods' nor 'fraction' provided.".to_string());
                                            }

                                            match &responsibility.time_periods {
                                                Some(time_periods) => {
                                                    for time_period in time_periods.to_vec(&[]) {
                                                        let timespan =
                                                            parse_time_span(time_period.as_str())
                                                                .expect(
                                                                "Erroneous timespan in manifest.",
                                                            );
                                                        let periods =
                                                            timespan.instantiate_periods(weekday);

                                                        for (day_offset, start, end) in periods {
                                                            coords.weekday =
                                                                weekday_plus(weekday, day_offset);

                                                            let coverage =
                                                                TemporalCoverageUnit::create(
                                                                    start,
                                                                    end,
                                                                    rotation_description
                                                                        .rotation
                                                                        .to_string(),
                                                                    weekday, //the NOMINAL weekday
                                                                );

                                                            match self.add_coverage(
                                                                &coords,
                                                                CoverageUnit::Temporal(coverage),
                                                            ) {
                                                                Ok(_) => (),
                                                                Err(e) => {
                                                                    return Err(e);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                None => (),
                                            }

                                            match &responsibility.weekly_fraction {
                                                Some(fraction) => {
                                                    coords.weekday = weekday;
                                                    let coverage = FractionalCoverageUnit::create(
                                                        rotation_description.rotation.to_string(),
                                                        weekday,
                                                        fraction.to_owned(),
                                                    );
                                                    self.add_coverage(
                                                        &coords,
                                                        CoverageUnit::WeekFraction(coverage),
                                                    )?;
                                                }
                                                None => (),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                None => (),
            };
        }

        Ok(())
    }

    fn foreach(
        &mut self,
        mut func: impl FnMut(&CoverageCoordinates, &mut CoverageAndWorkDay),
    ) {
        for (site, subspecialtymap) in self.map.iter_mut() {
            for (subspecialty, contextmap) in subspecialtymap.map.iter_mut() {
                for (context, modalitymap) in contextmap.map.iter_mut() {
                    for (modality, weekdaymap) in modalitymap.map.iter_mut() {
                        for (weekday, coverage_and_workday) in weekdaymap.map.iter_mut() {
                            let coords = CoverageCoordinates {
                                site: site.to_string(),
                                subspecialty: subspecialty.to_string(),
                                context: context.to_string(),
                                modality: modality.to_string(),
                                weekday: *weekday,
                            };

                            func(&coords, coverage_and_workday);
                        }
                    }
                }
            }
        }
    }

    pub fn audit(&mut self) -> HashMap<CoverageCoordinates, CoverageError> {
        let mut retval: HashMap<CoverageCoordinates, CoverageError> = HashMap::new();

        //let testcoords=testcoords();

        let func =
            |coords: &CoverageCoordinates, coverage_and_workday: &mut CoverageAndWorkDay| {
                let errs = coverage_and_workday.audit_coverage();

                retval.insert(coords.to_owned(), errs);
            };

        self.foreach(func);

        retval
    }

    pub fn analyze(&mut self) -> HashMap<String, HashMap<chrono::Weekday, AnalysisDatum>> {
        let mut retval: HashMap<String, HashMap<chrono::Weekday, AnalysisDatum>> = HashMap::new();

        let mut addfunc = |rotation: String, weekday: chrono::Weekday, data: AnalysisDatum| {
            let daymap: &mut HashMap<chrono::Weekday, AnalysisDatum> = match retval.entry(rotation)
            {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(empty) => {
                    let entry = empty.insert(HashMap::new());
                    entry
                }
            };

            let datum: &mut AnalysisDatum = match daymap.entry(weekday) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(empty) => {
                    let entry = empty.insert(AnalysisDatum {
                        total_bvu: 0.0,
                        total_rvu: 0.0,
                    });
                    entry
                }
            };

            *datum += data;
        };

        let func =
            |_coords: &CoverageCoordinates, coverage_and_workday: &mut CoverageAndWorkDay| {
                match &coverage_and_workday.coverages {
                    Some(coverage) => match coverage {
                        Coverage::Temporal(coverages) => {
                            for coverage in coverages {
                                let collection = coverage.collect_work(coverage_and_workday);
                                addfunc(coverage.get_rotation(), coverage.get_day(), collection);
                            }
                        }
                        Coverage::Fractional(coverages) => {
                            for coverage in coverages {
                                let collection = coverage.collect_work(coverage_and_workday);
                                addfunc(coverage.get_rotation(), coverage.get_day(), collection);
                            }
                        }
                    },
                    None => {
                        eprintln!("Uncovered work!");
                        panic!("Uncovered work!");
                    }
                }
            };

        self.foreach(func);

        retval
    }

    pub fn analysis_to_file(&mut self, path: String, use_rvu: bool) {
        let analysis = self.analyze();
        let mut writer = match csv::WriterBuilder::new()
            .delimiter(b',')
            .quote(b'"')
            .has_headers(false) //write manually
            .from_path(path)
        {
            Ok(x) => x,
            Err(_) => {
                panic!();
            }
        };

        let mut headers: Vec<String> = Vec::new();
        headers.push("".to_string());
        for weekday in ALL_DAYS {
            headers.push(weekday.to_string());
        }
        match writer.write_record(headers) {
            Ok(_) => (),
            Err(_) => panic!(),
        }

        let mut rotations: Vec<String> = Vec::new();
        for key in analysis.keys() {
            rotations.push(key.to_string());
        }
        rotations.sort();

        for rotation in &rotations {
            let daymap = analysis.get(rotation).expect("Should be a key");
            let mut v: Vec<String> = Vec::new();
            v.push(rotation.to_string());

            for weekday in ALL_DAYS {
                let val = match daymap.get(weekday) {
                    Some(x) => {
                        if use_rvu {
                            x.total_rvu.to_string()
                        } else {
                            x.total_bvu.to_string()
                        }
                    }
                    None => "".to_string(),
                };
                v.push(val);
            }

            match writer.write_record(v) {
                Ok(_) => (),
                Err(_) => panic!(),
            }
        }
    }

    pub fn audit_to_stream<T: Write>(&mut self, writer: &mut T) -> Result<(), std::io::Error> {
        let audit_result = self.audit();

        let mut sorted_keys: Vec<&CoverageCoordinates> = audit_result.keys().collect();
        sorted_keys.sort();

        for coords in sorted_keys {
            let errs = audit_result.get(coords).expect("Should be a key");
            match errs {
                CoverageError::NoWork => {
                    //Too verbose, skip these for now
                    //retval.push(format!("No work for: {site}, {subspecialty}, {context}, {modality}, {weekday}"));
                }
                CoverageError::NoCoverage(rvus) => {
                    writeln!(
                        writer,
                        "No coverage for: {}, {}, {}, {}, {} ({} rvus)",
                        coords.site,
                        coords.subspecialty,
                        coords.context,
                        coords.modality,
                        coords.weekday,
                        rvus
                    )?;
                }
                CoverageError::MalformedCoverage(errs) => {
                    if !errs.gaps.is_empty() {
                        for (rotation1, rotation2, desc, rvus) in &errs.gaps {
                            writeln!(
                                writer,
                                "Coverage gap: {}, {}, {}, {}, {}: {}-{} {} ({} rvus)",
                                coords.site,
                                coords.subspecialty,
                                coords.context,
                                coords.modality,
                                coords.weekday,
                                rotation1,
                                rotation2,
                                desc,
                                rvus
                            )?;
                        }
                    }
                    if !errs.overlaps.is_empty() {
                        for overlap in &errs.overlaps {
                            writeln!(
                                writer,
                                "Coverage overlap: {}, {}, {}, {}, {}: {}",
                                coords.site,
                                coords.subspecialty,
                                coords.context,
                                coords.modality,
                                coords.weekday,
                                overlap
                            )?;
                        }
                    }
                    match errs.incorrect_fraction {
                        Some(x) => {
                            writeln!(
                                writer,
                                "Incorrect fraction: {}, {}, {}, {}, {}: {}",
                                coords.site,
                                coords.subspecialty,
                                coords.context,
                                coords.modality,
                                coords.weekday,
                                x
                            )?;
                        }
                        None => (),
                    }
                }
            };
        }

        Ok(())
    }
}
impl<'a> CoordinateMap<'a, String, SubspecialtyMap> for CoverageMap {
    fn get_map(&mut self) -> &mut HashMap<String, SubspecialtyMap> {
        &mut self.map
    }
    fn get_coordinate(coords: &CoverageCoordinates) -> String {
        coords.site.clone()
    }
}
impl WorkCoverageMap for CoverageMap {
    fn add_work(&mut self, coords: &CoverageCoordinates, work: WorkUnit) {
        self.get_branch(coords).add_work(coords, work);
    }
    fn add_coverage(
        &mut self,
        coords: &CoverageCoordinates,
        coverage: CoverageUnit,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.get_branch(coords).add_coverage(coords, coverage)
    }
}
