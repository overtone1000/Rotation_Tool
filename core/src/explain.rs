use std::{collections::HashMap, error::Error};

use chrono::NaiveDateTime;

use crate::{
    categorization::buildSalemRVUMap,
    constraints::{
        exclude_site, is_after_this_hour, is_before_this_hour, is_not_holiday, is_this_day,
        only_these_subspecialties, only_this_context, ConstraintSet,
    },
    error::RotationToolError,
    globals::{Outpatient, MSK, NEURO_BRAIN, NEURO_OTHER, TPC},
    processed_source::ProcessedSource,
    rvu_map::{self, MapCoords, RVUMap},
};

pub(crate) fn explain_weekend_outpatient_volumes() -> Result<(), Box<dyn Error>> {
    let source = ProcessedSource::build()?;
    let rvu_map = buildSalemRVUMap(&source.main_data_table)?;

    let is_not_holiday_ref = &is_not_holiday;

    {
        let mut tcs: ConstraintSet<NaiveDateTime> = ConstraintSet::new();
        tcs.add(is_not_holiday_ref);
        let is_this_day_ref = &(is_this_day(chrono::Weekday::Fri));
        tcs.add(is_this_day_ref);
        let is_hour_ref = &(is_after_this_hour(17));
        tcs.add(is_hour_ref);

        ExplainTimeRegion("Friday after 5PM", &tcs, &source, &rvu_map)?;
    }

    {
        let mut tcs: ConstraintSet<NaiveDateTime> = ConstraintSet::new();
        tcs.add(is_not_holiday_ref);
        let is_this_day_ref = &(is_this_day(chrono::Weekday::Sat));
        tcs.add(is_this_day_ref);
        let is_hour_ref = &(is_before_this_hour(17));
        tcs.add(is_hour_ref);

        ExplainTimeRegion("Saturday before 5PM", &tcs, &source, &rvu_map)?;
    }

    {
        let mut tcs: ConstraintSet<NaiveDateTime> = ConstraintSet::new();
        tcs.add(is_not_holiday_ref);
        let is_this_day_ref = &(is_this_day(chrono::Weekday::Sun));
        tcs.add(is_this_day_ref);
        let is_hour_ref = &(is_before_this_hour(17));
        tcs.add(is_hour_ref);

        ExplainTimeRegion("Sunday before 5PM", &tcs, &source, &rvu_map)?;
    }

    {
        let mut tcs: ConstraintSet<NaiveDateTime> = ConstraintSet::new();
        tcs.add(is_not_holiday_ref);
        let is_this_day_ref = &(is_this_day(chrono::Weekday::Sun));
        tcs.add(is_this_day_ref);
        let is_hour_ref = &(is_after_this_hour(17));
        tcs.add(is_hour_ref);

        ExplainTimeRegion("Sunday after 5PM", &tcs, &source, &rvu_map)?;
    }

    Ok(())
}

pub(crate) extern "C" fn explain_weekday_variance() -> Result<(), Box<dyn Error>> {
    let source = ProcessedSource::build()?;
    let rvu_map = buildSalemRVUMap(&source.main_data_table)?;

    let weekdays = [
        chrono::Weekday::Mon,
        chrono::Weekday::Tue,
        chrono::Weekday::Wed,
        chrono::Weekday::Thu,
        chrono::Weekday::Fri,
    ];

    let is_not_holiday_ref = &is_not_holiday;

    for weekday in weekdays {
        let is_this_day_ref = &(is_this_day(weekday));

        {
            let mut tcs: ConstraintSet<NaiveDateTime> = ConstraintSet::new();
            tcs.add(is_not_holiday_ref);
            tcs.add(is_this_day_ref);
            let is_hour_ref = &(is_before_this_hour(15));
            tcs.add(is_hour_ref);

            ExplainTimeRegion(&format!("{} before 3PM", weekday), &tcs, &source, &rvu_map)?;
        }

        {
            let mut tcs: ConstraintSet<NaiveDateTime> = ConstraintSet::new();
            tcs.add(is_not_holiday_ref);
            tcs.add(is_this_day_ref);
            let is_hour_ref = &(is_after_this_hour(15));
            tcs.add(is_hour_ref);

            ExplainTimeRegion(&format!("{} after 3PM", weekday), &tcs, &source, &rvu_map)?;
        }
    }

    Ok(())
}

fn ExplainSegment(map: RVUMap) -> String {
    let mut total: f64 = 0.0;
    let mut neuro: f64 = 0.0;
    let mut msk: f64 = 0.0;

    let exclude_tpc_ref = &exclude_site(TPC.to_string());
    let only_outpatient_ref = &only_this_context(Outpatient.to_string());

    {
        let mut ccs: ConstraintSet<MapCoords> = ConstraintSet::new();
        ccs.add(exclude_tpc_ref);
        ccs.add(only_outpatient_ref);
        total = map.sliceAverageRVUs(Some(ccs));
    }

    {
        let mut subspecialties: Vec<String> = Vec::new();
        subspecialties.push(NEURO_BRAIN.to_string());
        subspecialties.push(NEURO_OTHER.to_string());

        let mut ccs: ConstraintSet<MapCoords> = ConstraintSet::new();
        ccs.add(exclude_tpc_ref);
        ccs.add(only_outpatient_ref);
        let only_these_subspecialties_ref = &only_these_subspecialties(subspecialties);
        ccs.add(only_these_subspecialties_ref);

        neuro = map.sliceAverageRVUs(Some(ccs));
    }

    {
        let mut subspecialties: Vec<String> = Vec::new();
        subspecialties.push(MSK.to_string());

        let mut ccs: ConstraintSet<MapCoords> = ConstraintSet::new();
        ccs.add(exclude_tpc_ref);
        ccs.add(only_outpatient_ref);
        let only_these_subspecialties_ref = &only_these_subspecialties(subspecialties);
        ccs.add(only_these_subspecialties_ref);

        msk = map.sliceAverageRVUs(Some(ccs));
    }

    format!(
        " RVU total={:.1} ({:.1} is Neuro, and {:.1} is MSK)",
        total, neuro, msk
    )
}

fn ExplainTimeRegion(
    desc: &str,
    date_inclusion: &ConstraintSet<NaiveDateTime>,
    source: &ProcessedSource,
    rvu_map: &HashMap<String, f64>,
) -> Result<(), Box<dyn Error>> {
    let map = match rvu_map::createMap(&source, &rvu_map, date_inclusion) {
        Ok(x) => x,
        Err(e) => {
            let err = RotationToolError::new(e);
            return Err(Box::new(err));
        }
    };
    println!("{}, {}", desc, ExplainSegment(map));
    Ok(())
}
