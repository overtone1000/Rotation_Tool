use std::collections::HashMap;

use chrono::{Datelike, Days, NaiveDate, Timelike};
use serde::Serialize;

use crate::{
    analysis::analysis_datum::{AnalysisDatum, WorkUnit},
    rotations::time_modifiers::{TimeSinceMidnight, NEXT_MIDNIGHT, THIS_MIDNIGHT},
};

use super::{
    coordinate::CoverageCoordinates, malformed_coverage::{CoverageError, MalformedCoverage}, units::{fractional_coverage::FractionalCoverageUnit, temporal_coverage::{weekday_plus, TemporalCoverageUnit}, Coverage, CoverageUnit}
};

#[derive(Debug, Default, Serialize)]
pub struct CoverageAndWorkDay {
    coverages: Option<Coverage>,
    work: Vec<WorkUnit>,
}

pub enum TimeAdjustment
{
    Fractional(chrono::Weekday),
    Temporal(i64)
}

impl TimeAdjustment
{
    pub fn get_weekday(&self, coords:&CoverageCoordinates)->chrono::Weekday
    {
        match self
        {
            TimeAdjustment::Fractional(weekday) => *weekday,
            TimeAdjustment::Temporal(offset) => weekday_plus(coords.weekday,*offset),
        }
    }

    pub fn get_date(&self, date:NaiveDate)->NaiveDate
    {
        match self
        {
            TimeAdjustment::Fractional(weekday) => {
                let mut shift:i64=i64::from(weekday.number_from_monday() as i64-date.weekday().number_from_monday() as i64);
                if shift<0 {shift+=7;}
                let date=date.checked_add_days(Days::new(shift as u64)).expect("Invalid date!");
                assert!(*weekday==date.weekday());
                date
            },
            TimeAdjustment::Temporal(offset) => {
                if *offset>0
                {
                    date.checked_add_days(Days::new(offset.abs().try_into().unwrap())).expect("Should be a valid date.")
                }
                else if *offset<0
                {
                    date.checked_sub_days(Days::new(offset.abs().try_into().unwrap())).expect("Should be a valid date.")
                }
                else {
                    date
                }
            }
        }
    }
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
    ) -> Vec<&WorkUnit> {
        let mut retval: Vec<&WorkUnit> = Vec::new();
        for work in &self.work {
            let tsm = TimeSinceMidnight::from_minutes(
                (work.get_datetime().num_seconds_from_midnight() / 60).into(),
            );
            if start <= tsm && tsm < end {
                retval.push(work);
            }
        }
        retval
    }

    fn aggregate_work_in_timespan(
        &self,
        start: TimeSinceMidnight,
        end: TimeSinceMidnight,
    ) -> AnalysisDatum {
        let mut retval: AnalysisDatum = AnalysisDatum::default();
        for work_unit in self.get_work_in_timespan(start, end) {
            retval.add_workunit(work_unit);
        }
        retval
    }

    pub fn total_rvus(
        &self
    )->f64
    {
        let mut retval:f64=0.0;
        for work in &self.work
        {
            retval+=work.get_absolute_rvu();
        }
        retval
    }

    fn collect_work_by_rotation_date(&self,coverage:&CoverageUnit)->HashMap<NaiveDate,AnalysisDatum>
    {
        let mut retval: HashMap<NaiveDate,AnalysisDatum> = HashMap::new();
        match coverage
        {
            CoverageUnit::Temporal(tcu) => {
                //Temporal work only collects work in a specific timespan
                for work in &self.get_work_in_timespan(tcu.start, tcu.end) {
                    let rotation_date=coverage.get_time_adjustment().get_date(work.get_datetime().date());
                    match retval.entry(rotation_date)
                    {
                        std::collections::hash_map::Entry::Occupied(mut occ) => 
                        {
                            occ.get_mut().add_workunit(work)
                        },
                        std::collections::hash_map::Entry::Vacant(vac) => 
                        {
                            let mut new_unit:AnalysisDatum=AnalysisDatum::default();
                            new_unit.add_workunit(work);
                            vac.insert(new_unit);
                        },
                    };
                }
            },
            CoverageUnit::WeekFraction(fcu) => {
                //Fractional work is for all the work in this coverage and workday but is scaled
                for work in &self.work {
                    let rotation_date=coverage.get_time_adjustment().get_date(work.get_datetime().date()); //Each fractional coverage is a separate entry so it only corresponds to a single date
                    match retval.entry(rotation_date)
                    {
                        std::collections::hash_map::Entry::Occupied(mut occ) => 
                        {
                            occ.get_mut().add_workunit(work)
                        },
                        std::collections::hash_map::Entry::Vacant(vac) => 
                        {
                            let mut new_unit:AnalysisDatum=AnalysisDatum::default();
                            new_unit.add_workunit(work);
                            vac.insert(new_unit);
                        },
                    };
                }
                //Fractional work is scaled
                for ad in retval.values_mut()
                {
                    ad.scale(fcu.get_fraction());
                }
            },
        };
        retval
    }

    fn for_each_coverage_unit<T>(&self, mut fun:T)->()
    where T:FnMut(&CoverageUnit)->()
    {
        for coverage in &self.coverages
        {
            let cu_iter:Vec<CoverageUnit> = match coverage
            {
                Coverage::Temporal(tcus) => {
                    tcus.iter().map(|tcu|{CoverageUnit::Temporal(tcu.clone())}).collect()
                },
                Coverage::Fractional(fcus) => {
                    fcus.iter().map(|fcu|{CoverageUnit::WeekFraction(fcu.clone())}).collect()
                },
            };

            for cu in cu_iter
            {
                fun(&cu);
            }
        }
    }

    pub fn for_each_analysis_datum_aggregate_and_average<T>(&self, mut fun:T)->()
    where T:FnMut(AnalysisDatum,&CoverageUnit)->()
    {
        self.for_each_coverage_unit(
            |coverage:&CoverageUnit|
            {
                let collected_by_date=self.collect_work_by_rotation_date(&coverage);
                let mut aggregate=AnalysisDatum::default();
                for ad in collected_by_date.values()
                {
                    aggregate+=ad.clone();                 
                }
                let denom:f64=f64::from(collected_by_date.keys().len() as u32);
                aggregate.scale(1.0/denom);
                fun(aggregate,coverage);
            }
        );
    }

    pub fn for_each_analysis_datum_by_rotation_date<T>(&self,mut fun:T)->()
    where T:FnMut(NaiveDate,AnalysisDatum,&CoverageUnit)->()
    {
        self.for_each_coverage_unit(
            |coverage:&CoverageUnit|
            {
                for (date,ad) in self.collect_work_by_rotation_date(&coverage)
                {
                    fun(date,ad,&coverage);
                }
            }
        );
    }

    pub fn audit_coverage(&mut self) -> CoverageError {
        self.sort_coverage();

        match &self.coverages {
            None => CoverageError::NoCoverage(
                self.aggregate_work_in_timespan(THIS_MIDNIGHT, NEXT_MIDNIGHT)
                    .get_rvu(),
            ),
            Some(coverages) => {
                let mut retval = MalformedCoverage::default();

                retval.no_work = self.work.is_empty();

                match coverages {
                    Coverage::Temporal(temporal_coverages) => {
                        match temporal_coverages.split_first() {
                            Some((mut farthest_unit, rest)) => {
                                //Check from midnight
                                if farthest_unit.starts_after_this_midnight() {
                                    let rvus = &self.aggregate_work_in_timespan(
                                        THIS_MIDNIGHT,
                                        farthest_unit.start,
                                    );
                                    retval.gaps.push((
                                        THIS_MIDNIGHT,
                                        farthest_unit.start,
                                        farthest_unit.to_string() + " starts after midnight",
                                        rvus.get_rvu(),
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
                                        let rvus = &self.aggregate_work_in_timespan(
                                            farthest_unit.end,
                                            cu.start,
                                        );
                                        retval.gaps.push((
                                            farthest_unit.end,
                                            cu.start,
                                            TemporalCoverageUnit::get_overlap_desc(
                                                farthest_unit,
                                                cu,
                                            ),
                                            rvus.get_rvu(),
                                        ));
                                    }

                                    //Adjust prior_end
                                    if cu.ends_after_other(farthest_unit) {
                                        farthest_unit = cu;
                                    }
                                }
                                //Check through midnight
                                if farthest_unit.ends_before_next_midnight() {
                                    let rvus = &self.aggregate_work_in_timespan(
                                        farthest_unit.end,
                                        NEXT_MIDNIGHT,
                                    );
                                    retval.gaps.push((
                                        farthest_unit.end,
                                        NEXT_MIDNIGHT,
                                        farthest_unit.to_string() + " ends before midnight",
                                        rvus.get_rvu(),
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
