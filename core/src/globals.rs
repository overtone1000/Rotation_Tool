pub mod file_names {
    pub(crate) const MAIN_DATA_FILE: &str = "./data/2024 to today.csv";
    pub(crate) const BVU_DATA_FILE: &str = "./data/BVU.csv";
    pub(crate) const BVU_UPDATE_FILE: &str = "./err/Unaccounted BVU Codes.csv";
    pub(crate) const CATEGORIES_LOCATION_FILE: &str = "./categories/Categories_Location.csv";
    pub(crate) const CATEGORIES_EXAM_FILE: &str = "./categories/Categories_Exam.csv";
    pub(crate) const UNACCOUNTED_EXAM_CODES_FILE: &str = "./err/Unaccounted_Exam_Codes.csv";
    pub(crate) const EXAMPLE_ROTATION_DESCRIPTIONS: &str = "./rotations/Example.yml";
    pub(crate) const SOURCE_CACHE: &str = "./cache/ProcessedSource.yml";
    pub(crate) const COVERAGE_AUDIT_OUT: &str = "./err/Coverage_Audit.tsv";
    pub(crate) const COVERAGE_AUDIT_NOWORK_OUT: &str = "./err/Coverage_Audit (no work).tsv";
    pub(crate) const COVERAGE_ANALYSIS_OUT: &str = "./output/WeekAnalysis";
    pub(crate) const VOLUME_BY_DATE_ROTATION: &str = "volume_by_date_and_rotation";
    pub(crate) const EXAM_ALIAS_FILE: &str = "./categories/Exam_Aliases.csv";
}

pub mod main_headers {
    pub(crate) enum PertinentHeaders {
        Accession,
        ProcedureCode,
        Exam,
        Location,
        ScheduledDatetime,
        Rvu,
        SiteID
        //Modality, //Not using this anymore
    }

    impl PertinentHeaders {
        pub(crate) fn get_label(&self) -> String {
            match self {
                PertinentHeaders::Accession => "Accession".to_string(),
                PertinentHeaders::ProcedureCode => "ProcedureCodeList".to_string(),
                PertinentHeaders::Exam => "ProcedureDescList".to_string(),
                PertinentHeaders::Location => "LocationDescription".to_string(),
                PertinentHeaders::ScheduledDatetime => "Exam Started".to_string(),
                PertinentHeaders::Rvu => "WorkRVU".to_string(),
                PertinentHeaders::SiteID => "SiteID".to_string()
                //PertinentHeaders::Modality => "Modality".to_string(),
            }
        }
    }
}

pub mod tpc_headers {
    pub(crate) enum PertinentHeaders {
        NumberIn2022,
        ExamCode,
    }

    impl PertinentHeaders {
        pub(crate) fn get_label(&self) -> String {
            match self {
                PertinentHeaders::NumberIn2022 => "2022 Volume".to_string(),
                PertinentHeaders::ExamCode => "Exam Code Translation".to_string(),
            }
        }
    }
}

pub mod bvu_headers {
    pub(crate) enum PertinentHeaders {
        ExamCode,
        TargetPercentile,
        ExamDescription,
        Comments,
    }

    impl PertinentHeaders {
        pub(crate) fn get_label(&self) -> String {
            match self {
                PertinentHeaders::TargetPercentile => "50th".to_string(),
                PertinentHeaders::ExamCode => "Location group".to_string(),
                PertinentHeaders::ExamDescription => "Exam Description".to_string(),
                PertinentHeaders::Comments => "Comments".to_string(),
            }
        }
    }
}

//facilities
pub const SH: &str = "SH";
const SC: &str = "SC";
const WB: &str = "WB";
const SRC: &str = "SRC";
pub const TPC: &str = "TPC";
pub const WVH: &str = "WVH";

//locations
const DXR: &str = "DXR";
const BC: &str = "BC";

//Sites are SH, SRC, SC, and TPC, but Facilities breaks down SH into its parts (SH, WVH, WB)...leaving out ST (Hope Ortho?), SV (?)
pub const FACILITIES: &[&str] = &[SH, SC, SRC, WVH, WB, TPC];

pub const SH_site_id:u64=1;
pub const SC_site_id:u64=4;
pub const SRC_site_id:u64=7;
pub const TPC_site_id:u64=8;

pub fn siteid_to_sitename(site_id:u64)->Option<String>{
    match site_id {
        //SH_site_id=>Some(SH.to_string()), //But could be WB or WVH! So, just ignore.
        SC_site_id=>Some(SC.to_string()),
        SRC_site_id=>Some(SRC.to_string()),
        TPC_site_id=>Some(TPC.to_string()),
        _=>None
    }
}

pub(crate) const MSK: &str = "MSK";
pub(crate) const MSK_WE_AH0C: &str = "MSK Weekend AH0C";
pub(crate) const NEURO: &str = "Neuro";

pub const NON_RADIOLOGY:&str = "Non-Radiology";

pub const SUBSPECIALTIES: &[&str] = &[
    "General XR",
    "General US",
    "General CT",
    "NM",
    "PET",
    "Body MR",
    "US Procedure (General)",
    "US Procedure (MSK)",
    "US Procedure (IR)",
    "US Procedure (IR or PA)",
    "Fluoro (General)",
    "Fluoro Procedure (MSK)",
    "Mamm (Screening)",
    "Mamm (Diag)",
    "Mamm (Procedures)",
    "Complex CTA+MRA",
    "Angio",
    "Vascular US",
    "CT Procedure",
    MSK,
    MSK_WE_AH0C,
    NEURO,
    "Intraop Fluoro",
    "Cardiac",
    "CT Colonography",
    "Breast MR",
    NON_RADIOLOGY
];

const INPATIENT: &str = "Inpatient";
pub const OUTPATIENT: &str = "Outpatient";
const ED: &str = "ED";

pub const CONTEXTS: &[&str] = &[INPATIENT, OUTPATIENT, ED, "Wet Read",NON_RADIOLOGY];

//modalities
const XR: &str = "XR";
const MG: &str = "MG";
const ANG: &str = "ANG";
const US: &str = "US";
const PET: &str = "PET";

pub const MODALITIES: &[&str] = &[
    XR, "CT", US, "MR", "NM", PET, "DEXA", "RF", MG, "XA", "CVUS", ANG, "CLINIC",
];

/*Patient class
Non-Radiology: 1,2
Outpatient: 2,5,8,
ED: 1,2,4,6,8
Inpatient: 0,1,6
*/

pub fn map_site_to_context(site: &str) -> Option<String> {
    match site {
        SH => Some(OUTPATIENT.to_string()),
        SC => Some(OUTPATIENT.to_string()),
        WB => Some(OUTPATIENT.to_string()),
        _ => None,
    }
}

pub fn map_SH_location_to_facility(location:&str)->Option<String> {
    match location
    {
        "OPRAD"=>Some(SH.to_string()),
        location=>{
            match &location[0..2]
            {
                "WV"=>Some(WVH.to_string()),
                _=>None
            }
        }
    }
}

pub fn get_modality_alias(modality: &String) -> Option<String> {
    match modality.as_str() {
        "MAM" => Some(MG.to_string()),
        "CR" => Some(XR.to_string()),
        "PT" => Some(PET.to_string()),
        _ => None,
    }
}

pub fn get_modality_from_procedure_desc(desc: String) -> Option<String> {
    match desc.as_str() {
        "ANG PA LYSIS" => Some(ANG.to_string()),
        "ANG NEPHROSTOMY REMOVAL" => Some(ANG.to_string()),
        "US GUIDANCE NEXPLANON REMOVAL" => Some(US.to_string()),
        "MAM MAGSEED PLACEMENT" => Some(MG.to_string()),
        "US MAGSEED PLACEMENT" => Some(US.to_string()),
        "ANG TEMP DIALYSIS CATHETER PLACEMENT" => Some(ANG.to_string()),
        _ => None,
    }
}

pub fn get_location_site_mapping(location: &String) -> Option<String> {
    match location.as_str() {
        DXR => Some(SH.to_string()),
        BC => Some(SH.to_string()),
        _ => None,
    }
}

pub const BUSINESS_DAYS: &[&chrono::Weekday] = &[
    &chrono::Weekday::Mon,
    &chrono::Weekday::Tue,
    &chrono::Weekday::Wed,
    &chrono::Weekday::Thu,
    &chrono::Weekday::Fri,
];

pub const ALL_DAYS: &[&chrono::Weekday] = &[
    &chrono::Weekday::Mon,
    &chrono::Weekday::Tue,
    &chrono::Weekday::Wed,
    &chrono::Weekday::Thu,
    &chrono::Weekday::Fri,
    &chrono::Weekday::Sat,
    &chrono::Weekday::Sun,
];
