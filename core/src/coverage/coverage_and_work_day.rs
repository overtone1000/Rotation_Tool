use chrono::Timelike;
use serde::Serialize;

use crate::{
    analysis::analysis_datum::{AnalysisDatum, WorkUnit},
    rotations::time_modifiers::{TimeSinceMidnight, NEXT_MIDNIGHT, THIS_MIDNIGHT},
};

use super::{
    malformed_coverage::{CoverageError, MalformedCoverage},
    units::{temporal_coverage::TemporalCoverageUnit, Coverage, CoverageUnit},
};

#[derive(Debug, Default, Serialize)]
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

    pub fn aggregate_work_in_timespan(
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
