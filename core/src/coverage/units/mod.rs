use std::{
    collections::{hash_map::Entry, HashMap},
    ops::AddAssign,
};

use chrono::NaiveDate;
use serde::Serialize;

use crate::{analysis::analysis_datum::AnalysisDatum, error::source_error::SourceError};

use self::{fractional_coverage::FractionalCoverageUnit, temporal_coverage::TemporalCoverageUnit};

use super::{coverage_and_work_day::CoverageAndWorkDay};

pub(crate) mod fractional_coverage;
pub(crate) mod temporal_coverage;

#[derive(Clone, Debug)]
pub enum CoverageUnit {
    Temporal(TemporalCoverageUnit),
    WeekFraction(FractionalCoverageUnit),
}

#[derive(Debug, Serialize)]
pub enum Coverage {
    Temporal(Vec<TemporalCoverageUnit>),
    Fractional(Vec<FractionalCoverageUnit>),
}

impl Coverage {
    fn coverage_error(coverage: &CoverageUnit, coverages: &Coverage) -> String {
        let message = format!("Mixing fractional and temporal coverage types is not allowed. This was attempted for {:?} with the following coverages already listed {:?}",coverage,coverages);
        message
    }
    pub fn add(&mut self, coverage: CoverageUnit) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Coverage::Temporal(coverages) => match coverage {
                CoverageUnit::Temporal(new_coverage) => {
                    coverages.push(new_coverage);
                    coverages.sort();
                }
                CoverageUnit::WeekFraction(new_coverage) => {
                    return SourceError::generate_boxed(Self::coverage_error(
                        &(CoverageUnit::WeekFraction(new_coverage)),
                        self,
                    ));
                }
            },
            Coverage::Fractional(coverages) => match coverage {
                CoverageUnit::Temporal(new_coverage) => {
                    return SourceError::generate_boxed(Self::coverage_error(
                        &(CoverageUnit::Temporal(new_coverage)),
                        self,
                    ));
                }
                CoverageUnit::WeekFraction(new_coverage) => {
                    coverages.push(new_coverage);
                    coverages.sort();
                }
            },
        }
        Ok(())
    }
}
