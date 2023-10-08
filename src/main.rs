#![allow(unused_parens)]

use std::{error::Error, collections::{HashSet, HashMap}, io::{ErrorKind, Write}, fs, path::Path, str::FromStr};

use chrono::{DateTime, Local, NaiveDateTime, Datelike, NaiveDate};
use main_headers::pertinent_headers;
use static_categorization::sites;

use crate::dates::checkHoliday;

mod table;
mod dates;

mod file_names
{
    pub(crate) const MAIN_DATA_FILE:&str = "./data/SRC_SC_SH_WVH_WB Business Day Data.csv";
    pub(crate) const CATEGORIES_LOCATION_FILE:&str = "./categories/Categories_Location.csv";
    pub(crate) const CATEGORIES_EXAM_FILE:&str = "./categories/Categories_Exam.csv";
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

    impl pertinent_headers {
        pub(crate) fn getLabel(&self)->String
        {
            match self{
                pertinent_headers::accession => "Accession".to_string(),
                pertinent_headers::procedure_code => "ProcedureCodeList".to_string(),
                pertinent_headers::exam => "ProcedureDescList".to_string(),
                pertinent_headers::location => "LocationDescription".to_string(),
                pertinent_headers::scheduled_datetime => "Exam Started".to_string(),
                pertinent_headers::rvu => "WorkRVU".to_string(),
                pertinent_headers::modality => "Modality".to_string(),
            }
        }
    }
}

mod exam_categories {
    use std::cmp::Ordering;

    pub(crate) enum pertinent_headers {
        procedure_code,
        exam,
        subspecialty,
        comments
    }

    impl pertinent_headers {
        pub(crate) fn getLabel(&self)->String
        {
            match self{
                pertinent_headers::procedure_code => "Exam Code".to_string(),
                pertinent_headers::exam => "Exam Description".to_string(),
                pertinent_headers::subspecialty => "Subspecialty".to_string(),
                pertinent_headers::comments => "Comments".to_string(),
            }
        }
    }
    
    pub(crate) struct exam_category {
        pub procedure_code:String,
        pub exam:String,
        pub subspecialty:String,
        pub comments:String
    }

    impl Eq for exam_category {}

    impl PartialOrd for exam_category {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl PartialEq for exam_category {
        fn eq(&self, other: &Self) -> bool {
            self.procedure_code == other.procedure_code &&
            self.exam == other.exam
            //self.subspecialty == other.subspecialty &&
            //self.comments == other.comments
        }
    }

    impl Ord for exam_category {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self.exam.cmp(&other.exam)
            {
                std::cmp::Ordering::Equal => self.procedure_code.cmp(&other.procedure_code),
                (examcmp) => examcmp
            }
        }
    }
}

mod location_categories {
    use std::cmp::Ordering;

    pub(crate) enum pertinent_headers {
        location,
        context,
        comments
    }

    impl pertinent_headers {
        pub(crate) fn getLabel(&self)->String
        {
            match self{
                pertinent_headers::location => "Location".to_string(),
                pertinent_headers::context => "Context".to_string(),
                pertinent_headers::comments => "Comments".to_string(),
            }
        }
    }
    
    pub(crate) struct location_category {
        pub location:String,
        pub context:String,
        pub comments:String
    }

    impl Eq for location_category {}

    impl PartialOrd for location_category {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl PartialEq for location_category {
        fn eq(&self, other: &Self) -> bool {
            self.location == other.location
            //self.context == other.context &&
            //self.comments == other.comments
        }
    }

    impl Ord for location_category {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.location.cmp(&other.location)
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

fn get_categories_list(
    main_data_table:&table::Table,
    exam_categories_table:&table::Table
)->Option<Vec<exam_categories::exam_category>>
{
    let main_exam_categories=main_data_table.getKeyedColumnSampleMap(
        &(main_headers::pertinent_headers::procedure_code.getLabel())
    );

    let existing_exam_categories=exam_categories_table.getKeyedColumnSampleMap(
        &(exam_categories::pertinent_headers::procedure_code.getLabel())
    );

    let mut complete_exam_code_list:Vec<exam_categories::exam_category>=Vec::new();
    
    for procedure_code in main_exam_categories.keys()
    {
        let mut next_member:exam_categories::exam_category=exam_categories::exam_category{
            procedure_code:procedure_code.to_string(),
            exam:"".to_string(),
            subspecialty:"".to_string(),
            comments:"".to_string()
        };

        match existing_exam_categories.get(procedure_code)
        {
            None=>{
                println!("Couldn't find {}",procedure_code.to_string());
                let sample_row_index = main_exam_categories.get(procedure_code)?;
                next_member.procedure_code=main_data_table.getVal(&main_headers::pertinent_headers::procedure_code.getLabel(), sample_row_index)?;
                next_member.exam=main_data_table.getVal(&main_headers::pertinent_headers::exam.getLabel(), sample_row_index)?;
            },
            Some(sample_row_index)=>{
                next_member.procedure_code=exam_categories_table.getVal(&exam_categories::pertinent_headers::procedure_code.getLabel(), sample_row_index)?;
                next_member.exam=exam_categories_table.getVal(&exam_categories::pertinent_headers::exam.getLabel(), sample_row_index)?;
                next_member.subspecialty=exam_categories_table.getVal(&exam_categories::pertinent_headers::subspecialty.getLabel(), sample_row_index)?;
                next_member.comments=exam_categories_table.getVal(&exam_categories::pertinent_headers::comments.getLabel(), sample_row_index)?;
            }
        }

        complete_exam_code_list.push(next_member);  
    }

    complete_exam_code_list.sort();

    return Some(complete_exam_code_list);
}

fn get_locations_list(
    main_data_table:&table::Table,
    exam_locations_table:&table::Table
)->Option<Vec<location_categories::location_category>>
{
    let main_exam_locations=main_data_table.getKeyedColumnSampleMap(
        &(main_headers::pertinent_headers::location.getLabel())
    );

    let existing_exam_locations=exam_locations_table.getKeyedColumnSampleMap(
        &(location_categories::pertinent_headers::location.getLabel())
    );

    let mut complete_exam_location_list:Vec<location_categories::location_category>=Vec::new();
    
    for location in main_exam_locations.keys()
    {
        if location == "300"
        {
            println!("Caught it");
        }
        let mut next_member:location_categories::location_category=location_categories::location_category{
            location:location.to_string(),
            context:"".to_string(),
            comments:"".to_string()
        };

        match existing_exam_locations.get(location)
        {
            None=>{
                println!("Couldn't find {}",location.to_string());
                let sample_row_index = main_exam_locations.get(location)?;
                next_member.location=main_data_table.getVal(&main_headers::pertinent_headers::location.getLabel(), sample_row_index)?;
            },
            Some(sample_row_index)=>{
                next_member.location=exam_locations_table.getVal(&location_categories::pertinent_headers::location.getLabel(), sample_row_index)?;
                next_member.context=exam_locations_table.getVal(&location_categories::pertinent_headers::context.getLabel(), sample_row_index)?;
                next_member.comments=exam_locations_table.getVal(&location_categories::pertinent_headers::comments.getLabel(), sample_row_index)?;
            }
        }

        complete_exam_location_list.push(next_member);  
    }

    complete_exam_location_list.sort();

    return Some(complete_exam_location_list);
}

fn backup(dt:DateTime<Local>,p:String,label:String)->Result<u64,std::io::Error>
{
    let backup_path="./categories/archive/".to_string() + &dt.timestamp().to_string() + " backup of " + &label;
    println!("Backup to {}",backup_path);
    return fs::copy(p.to_owned(),backup_path);
}

const SITES:&[&str]=
&[
    "SH",
    "SC",
    "SRC",
    "WVH",
    "WB",
    "TPC"
];

const SUBSPECIALTIES:&[&str]=
&[
    "General",
    "US Procedure (General)",
    "US Procedure (MSK)",
    "US Procedure (IR)",
    "US Procedure (IR or PA)",
    "Fluoro (General)",
    "Fluoro Procedure (MSK)",
    "Screening Mamm",
    "Diagnostic Mamm",
    "Complex CTA/MRA",
    "Angio",
    "Vascular US",
    "CT Procedure",
    "MSK",
    "Neuro (Brain)",
    "Neuro (Other)",
    "Intraop Fluoro"
];

const CONTEXTS:&[&str]=
&[
    "Inpatient",
    "Outpatient",
    "ED"
];

const MODALITIES:&[&str]=
&[
    "XR",
    "CT",
    "US",
    "MR",
    "NM",
    "PET",
    "DEXA",
    "RF",
    "MG",
    "XA",
    "CVUS",
    "ANG",
    "CLINIC"
];

struct MapEntry
{
    rvus:f32
}

struct RVUMap
{
    map:HashMap<String,HashMap<String,HashMap<String,HashMap<String,MapEntry>>>>,
    included_dates:HashSet<NaiveDate>
}

impl RVUMap
{
    fn new()->RVUMap
    {
        let mut retval = RVUMap{
            map:HashMap::new(),
            included_dates:HashSet::new()
        };

        for site in SITES
        {
            let mut sub_map:HashMap<String,HashMap<String,HashMap<String,MapEntry>>> = HashMap::new();
            for subspecialty in SUBSPECIALTIES
            {
                let mut con_map:HashMap<String,HashMap<String,MapEntry>> = HashMap::new();
                for context in CONTEXTS
                {
                    let mut mod_map:HashMap<String,MapEntry> = HashMap::new();
                    for modality in MODALITIES
                    {
                        mod_map.insert(modality.to_string(),MapEntry{rvus:0.0});
                    }
                    con_map.insert(context.to_string(),mod_map);
                }
                sub_map.insert(subspecialty.to_string(),con_map);
            }
            retval.map.insert(site.to_string(), sub_map);
        }

        return retval;
    }

    fn getEntry(&self,site:String,subspecialty:String,context:String,modality:String)->Option<&MapEntry>
    {
        return self.map.get(&site)?.get(&subspecialty)?.get(&context)?.get(&modality);
    }

}


fn createMap(main_data_table:&table::Table, exam_to_subspecialty_map:&HashMap<String,String>, location_to_context_map:&HashMap<String,String>)->Option<RVUMap>
{
    let mut rvumap = RVUMap::new();
    
    for row_i in main_data_table.rowIndices()
    {
        let datetimestring=main_data_table.getVal(&main_headers::pertinent_headers::scheduled_datetime.getLabel(), &row_i)?;
        
        let date=match NaiveDate::parse_from_str(&datetimestring, "%m/%d/%y %H:%M")
        {
            Ok(dt)=>{
                println!("{},{:?}",datetimestring,dt);
                dt
            },
            Err(e)=>{
                println!("Bad date {:?}",e);
                return None;
            }
        };

        if dates::checkWeekDay(date) && !dates::checkHoliday(date)
        {
            rvumap.included_dates.insert(date);
            
            let listed_site=main_data_table.getVal(&main_headers::pertinent_headers::accession.getLabel(), &row_i)?;
            for site in SITES
            {
                if (&listed_site[0..site.len()])==site.to_string()
                {

                }
            }

            let location=main_data_table.getVal(&main_headers::pertinent_headers::location.getLabel(), &row_i)?;
            let context=location_to_context_map.get(&location)?;
            let exam_code=main_data_table.getVal(&main_headers::pertinent_headers::exam.getLabel(), &row_i)?;
            let subspecialty=exam_to_subspecialty_map.get(&exam_code)?;
        }
    }

    Some(rvumap)
}

fn main()->Result<(), Box<dyn Error>> {
    let main_data_table=table::Table::create(file_names::MAIN_DATA_FILE)?;

    //Get current categories
    let mut exam_categories_table=table::Table::create(file_names::CATEGORIES_EXAM_FILE)?;
    let mut location_categories_table=table::Table::create(file_names::CATEGORIES_LOCATION_FILE)?;

    let exam_categories_list = match get_categories_list(&main_data_table,&exam_categories_table)
    {
        None=>{return Err("Couldn't build categories list.".into());},
        Some(categories_list)=>categories_list
    };

    exam_categories_table.clear();
    for category_row in exam_categories_list.as_slice()
    {
        let mut newrow:Vec<String>=Vec::new();
        newrow.push(category_row.procedure_code.to_owned());
        newrow.push(category_row.exam.to_owned());
        newrow.push(category_row.subspecialty.to_owned());
        newrow.push(category_row.comments.to_owned());
        exam_categories_table.pushrow(newrow);
    }

   
    let location_categories_list = match get_locations_list(&main_data_table,&location_categories_table)
    {
        None=>{return Err("Couldn't build location list.".into());},
        Some(categories_list)=>categories_list
    };


    location_categories_table.clear();
    for location_row in location_categories_list.as_slice()
    {
        let mut newrow:Vec<String>=Vec::new();
        newrow.push(location_row.location.to_owned());
        newrow.push(location_row.context.to_owned());
        newrow.push(location_row.comments.to_owned());
        location_categories_table.pushrow(newrow);
    }
        
    let dt = chrono::offset::Local::now();

    //Archive and save new file if changed
    match backup(dt,file_names::CATEGORIES_EXAM_FILE.to_string(),"Categories_Exam.csv".to_owned())
    {
        Ok(_)=>{
            exam_categories_table.write_to_file(file_names::CATEGORIES_EXAM_FILE.to_owned());
        },
        Err(x)=>{
            println!("{}",x);
            return Err(Box::new(x));
        }
    }
    match backup(dt,file_names::CATEGORIES_LOCATION_FILE.to_string(),"Categories_Location.csv".to_owned())
    {
        Ok(_)=>{
            location_categories_table.write_to_file(file_names::CATEGORIES_LOCATION_FILE.to_owned());
        },
        Err(x)=>{
            println!("{}",x);
            return Err(Box::new(x));
        }
    }

    createMap(&main_data_table,&exam_categories_list,&location_categories_list);

    println!("Finished.");
    return Ok(());
}
