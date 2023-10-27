use std::{error::Error, collections::HashMap};

use chrono::NaiveDateTime;

use crate::{ProcessedSource, categorization::buildSalemRVUMap, rvu_map::{RVUMap, self}, error::RotationToolError};


pub(crate) fn explain()->Result<(), Box<dyn Error>>
{
    let source=ProcessedSource::build()?;
    let rvu_map=buildSalemRVUMap(&source.main_data_table)?;

    //ExplainTimeRegion("Friday before 5PM",Friday5PM_to_Saturday12AM,&source,&rvu_map)?;
    //ExplainTimeRegion("Saturday before 5PM",SaturdayBefore5PM,&source,&rvu_map)?;
    //ExplainTimeRegion("Sunday before 5PM",SundayBefore5PM,&source,&rvu_map)?;
    //ExplainTimeRegion("Sunday after 5PM",SundayAfter5PM,&source,&rvu_map)?;

    Ok(())
}


fn ExplainSegment(map:RVUMap)->String{
    let total=map.sliceAverageRVUs(Some(isOutpatient),None);
    let neuro=map.sliceAverageRVUs(Some(isOutpatientNeuro),None);
    let msk=map.sliceAverageRVUs(Some(isOutpatientMSK),None);



    format!(" RVU total={:.1} ({:.1} is Neuro, and {:.1} is MSK)",total,neuro,msk)
}


fn ExplainTimeRegion(desc:&str, date_inclusion:fn(datetime:NaiveDateTime)->bool, source:&ProcessedSource, rvu_map:&HashMap<String, f64>)->Result<(), Box<dyn Error>>
{
    let map = match rvu_map::createMap(&source,&rvu_map,date_inclusion)
    {
        Ok(x)=>x,
        Err(e)=>{
            let err=RotationToolError::new(e);
            return Err(Box::new(err));
        }
    };
    println!("{}, {}", desc, ExplainSegment(map));
    Ok(())
}
