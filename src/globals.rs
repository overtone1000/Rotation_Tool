
pub mod file_names
{
    pub(crate) const MAIN_DATA_FILE:&str = "./data/SRC_SC_SH_WVH_WB Business Day Data.csv";
    pub(crate) const CATEGORIES_LOCATION_FILE:&str = "./categories/Categories_Location.csv";
    pub(crate) const CATEGORIES_EXAM_FILE:&str = "./categories/Categories_Exam.csv";
}

pub mod main_headers {
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

//sites
const SH:&str="SH";
const SC:&str="SC";
const WB:&str="WB";

//locations
const DXR:&str="DXR";
const BC:&str="BC";

//modalities

const XR:&str="XR";
const MG:&str="MG";

pub const ignored:&str="Ignored";

pub const SITES:&[&str]=
&[
    SH,
    SC,
    "SRC",
    "WVH",
    WB,
    "TPC"
];

pub const SUBSPECIALTIES:&[&str]=
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

pub const CONTEXTS:&[&str]=
&[
    "Inpatient",
    "Outpatient",
    "ED"
];



pub const MODALITIES:&[&str]=
&[
    XR,
    "CT",
    "US",
    "MR",
    "NM",
    "PET",
    "DEXA",
    "RF",
    MG,
    "XA",
    "CVUS",
    "ANG",
    "CLINIC"
];

pub fn mapSiteToContext(site:&str) -> Option<&str>{
    let sh=SH;
    let sc=SC;
    let wb=WB;
    match site
    {
        sh => Some("Outpatient"),
        sc => Some("Outpatient"),
        wb => Some("Outpatient"),
        _ => None
    }
}

pub fn getModalityAlias(modality:&str) -> Option<&str>{
    let mg=MG;
    let xr=XR;
    match modality
    {
        mg => Some("MAM"),
        xr => Some("CR"),
        _ => None
    }
}

pub fn getLocationSiteMapping(location:String)->Option<String>
{
    let dxr=DXR;
    let bc=BC;

    match location
    {
        dxr=>Some(SH.to_string()),
        bc=>Some(SH.to_string()),
        (_)=>None
    }
}