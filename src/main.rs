#![allow(unused_parens)]

use std::fs;
use std::vec::Vec;
use std::error::Error;

mod file_names
{
    pub(crate) const MAIN_DATA_FILE:&str = "./data/SRC_SC_SH_WVH_WB Business Day Data.tsv";
}

mod main_headers {
    pub(crate) enum pertinent_headers {
        accession,
        procedure_code,
        exam,
        location,
        scheduled_datetime,
        rvu,
        modality
    }

    pub(crate) fn getFromLabel(header:&pertinent_headers)->&str{
        match header{
            pertinent_headers::accession => "Accession",
            pertinent_headers::procedure_code => "ProcedureCodeList",
            pertinent_headers::exam => "ProcedureDescList",
            pertinent_headers::location => "LocationDescription",
            pertinent_headers::scheduled_datetime => "Exam Started",
            pertinent_headers::rvu => "WorkRVU",
            pertinent_headers::modality => "Modality",
        }
    }
}

mod time {
    pub(crate) const time_start_hour:i32=6;
    pub(crate) const time_start_minute:i32=0;
    pub(crate) const time_step_minutes:i32=30;
    pub(crate) fn time_row_count()->i32{
        return (((24.0*60.0)/(time_step_minutes as f32))).floor() as i32;
    }
    pub(crate) fn getTimeRowIndex(hour:i32, minute:i32)->i32{
        let mut minute_of_day = hour*60+minute;
        let start_minute_of_day=time_start_hour*60+time_start_minute;
        if(minute_of_day<start_minute_of_day){minute_of_day+=24*60;}
        return (((minute_of_day-start_minute_of_day) as f32)/(time_step_minutes as f32)).floor() as i32;
    }
}

mod static_categorization {

    pub(crate) mod sites {
        pub(crate) const sh:&str="SH";
        pub(crate) const src:&str="SRC";
        pub(crate) const sc:&str="SC";
        pub(crate) const wvh:&str="WVH";
        pub(crate) const wb:&str="WB";
        pub(crate) const tpc:&str="TPC";
      }

      pub(crate) mod modalities{
        pub(crate) const xr:&str="XR";
        pub(crate) const ct:&str="CT";
        pub(crate) const us:&str="US";
        pub(crate) const mr:&str="MR";
        pub(crate) const nm:&str="NM";
        pub(crate) const pet:&str="PET";
        pub(crate) const dexa:&str="DEXA";
        pub(crate) const fluoro:&str="RF";
        pub(crate) const mg:&str="MG";
        pub(crate) const xa:&str="XA";
        pub(crate) const cvus:&str="CVUS";
        pub(crate) const angio:&str="ANG";
        pub(crate) const clinic:&str="CLINIC"; //ABI at SRC
    }

    pub(crate) mod contexts {
        pub(crate) const outpatient:&str="Outpatient";
        pub(crate) const inpatient:&str="Inpatient";
        pub(crate) const ed:&str="ED";
    }

    pub(crate) fn mapSiteToContext(site:&str) -> Option<&str>{
        match site
        {
            sites::sh => Some("Outpatient"),
            sites::sc => Some("Outpatient"),
            sites::wb => Some("Outpatient"),
            _ => None
        }
    }

    pub(crate) fn getModalityAliases(modality:&str) -> Option<&str>{
        match modality
        {
            modalities::mg => Some("MAM"),
            modalities::xr => Some("CR"),
            _ => None
        }
    }

    pub(crate) const ignored:&str="Ignored";
}

fn main()->Result<(), Box<dyn Error>> {
    
    let file=fs::read_to_string(file_names::MAIN_DATA_FILE)?;
    let rows=file.lines().clone();

    let mut headers:Option<Vec<&str>>=None;
    let mut data:Vec<Vec<&str>>=Vec::new();

    //Data rows
    for row in rows
    {
        let cells = row.rsplit("\t");
        let mut newrow:Vec<&str>=Vec::new();

        for cell in cells
        {
            newrow.push(cell);
        }

        match headers
        {
            None => headers=Some(newrow),
            Some(_) => data.push(newrow),
        }
    }
    
    return Ok(());
}
