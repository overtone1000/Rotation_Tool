use std::error::Error;

use crate::{
    analysis::analysis_datum::AnalysisDatum,
    coverage::{
        self, coordinate::CoverageCoordinates, coverage_and_work_day::{CoverageAndWorkDay, TimeAdjustment}, units::{Coverage, CoverageUnit}, work_coverage_map::maps::CoverageMap
    },
};

pub fn details(
    coverage_map: &mut CoverageMap,
    analyzed_weekday: chrono::Weekday,
    analyzed_rotation: &str,
) -> Result<AnalysisDatum, Box<dyn Error>> {
    let mut aggregate: AnalysisDatum = AnalysisDatum::default();

    let mut addfunc = |rotation: String, weekday: chrono::Weekday, data: AnalysisDatum| {
        if weekday == analyzed_weekday && rotation == analyzed_rotation {
            aggregate+=data;
        }
    };

    coverage_map.foreach_mut(
        |coords: &CoverageCoordinates, coverage_and_workday: &mut CoverageAndWorkDay|
        {
            coverage_and_workday.for_each_analysis_datum_aggregate_and_average(
                |ad:AnalysisDatum,cu:&CoverageUnit|
                {
                    addfunc(cu.get_rotation(),cu.get_time_adjustment().get_weekday(coords),ad);
                }
            );
        }
    );

    Ok(aggregate)
}
