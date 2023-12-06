pub mod file_names {
    pub(crate) const MAIN_DATA_FILE: &str = "./data/SRC_SC_SH_WVH_WB Business Day Data.csv";
    pub(crate) const TPC_DATA_FILE: &str = "./data/TPC Whole Year with Salem Code Map.csv";
    pub(crate) const BVU_DATA_FILE: &str = "./data/BVU.csv";
    pub(crate) const BVU_UPDATE_FILE: &str = "./data/BVU_modified.csv";
    pub(crate) const CATEGORIES_LOCATION_FILE: &str = "./categories/Categories_Location.csv";
    pub(crate) const CATEGORIES_EXAM_FILE: &str = "./categories/Categories_Exam.csv";
    pub(crate) const OUT_FILE: &str = "./output/Categorized Weekday RVU Map";
    pub(crate) const BVU_OUT_FILE: &str = "./output/Categorized Weekday BVU Map";
    pub(crate) const EXAMPLE_ROTATION_DESCRIPTIONS: &str = "./rotations/Example.yml";
    pub(crate) const SOURCE_CACHE: &str = "./cache/ProcessedSource.yml";
    pub(crate) const COVERAGE_AUDIT_OUT: &str = "./err/Coverage_Audit";
    pub(crate) const COVERAGE_ANALYSIS_OUT: &str = "./output/WeekAnalysis";
}

pub mod main_headers {
    pub(crate) enum pertinent_headers {
        accession,
        procedure_code,
        exam,
        location,
        scheduled_datetime,
        rvu,
        modality,
    }

    impl pertinent_headers {
        pub(crate) fn getLabel(&self) -> String {
            match self {
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

pub mod tpc_headers {
    pub(crate) enum pertinent_headers {
        number_in_2022,
        exam_code,
    }

    impl pertinent_headers {
        pub(crate) fn getLabel(&self) -> String {
            match self {
                pertinent_headers::number_in_2022 => "2022 Volume".to_string(),
                pertinent_headers::exam_code => "Exam Code Translation".to_string(),
            }
        }
    }
}

pub mod bvu_headers {
    pub(crate) enum pertinent_headers {
        exam_code,
        target_percentile,
        exam_description,
        comments,
    }

    impl pertinent_headers {
        pub(crate) fn getLabel(&self) -> String {
            match self {
                pertinent_headers::target_percentile => "50th".to_string(),
                pertinent_headers::exam_code => "Location group".to_string(),
                pertinent_headers::exam_description => "Exam Description".to_string(),
                pertinent_headers::comments => "Comments".to_string(),
            }
        }
    }
}

//sites
const SH: &str = "SH";
const SC: &str = "SC";
const WB: &str = "WB";
pub const TPC: &str = "TPC";

//locations
const DXR: &str = "DXR";
const BC: &str = "BC";

pub const SITES: &[&str] = &[SH, SC, "SRC", "WVH", WB, TPC];

pub(crate) const MSK: &str = "MSK";
pub(crate) const NEURO_BRAIN: &str = "Neuro (Brain)";
pub(crate) const NEURO_OTHER: &str = "Neuro (Other)";

pub const SUBSPECIALTIES: &[&str] = &[
    "General",
    "US Procedure (General)",
    "US Procedure (MSK)",
    "US Procedure (IR)",
    "US Procedure (IR or PA)",
    "Fluoro (General)",
    "Fluoro Procedure (MSK)",
    "Screening Mamm",
    "Diagnostic Mamm",
    "Mamm Procedure",
    "Complex CTA+MRA",
    "Angio",
    "Vascular US",
    "CT Procedure",
    MSK,
    NEURO_BRAIN,
    NEURO_OTHER,
    "Intraop Fluoro",
    "Cardiac",
    "CT Colonography",
    "Breast MR",
    "Non-Radiology",
];

const Inpatient: &str = "Inpatient";
pub const Outpatient: &str = "Outpatient";
const ED: &str = "ED";

pub const CONTEXTS: &[&str] = &[Inpatient, Outpatient, ED, "Wet Read"];

//modalities
const XR: &str = "XR";
const MG: &str = "MG";
const ANG: &str = "ANG";
const US: &str = "US";
const PET: &str = "PET";

pub const MODALITIES: &[&str] = &[
    XR, "CT", US, "MR", "NM", PET, "DEXA", "RF", MG, "XA", "CVUS", ANG, "CLINIC",
];

pub fn mapSiteToContext(site: &str) -> Option<String> {
    match site {
        SH => Some(Outpatient.to_string()),
        SC => Some(Outpatient.to_string()),
        WB => Some(Outpatient.to_string()),
        _ => None,
    }
}

pub fn getModalityAlias(modality: &String) -> Option<String> {
    let retval = match modality.as_str() {
        "MAM" => Some(MG.to_string()),
        "CR" => Some(XR.to_string()),
        "PT" => Some(PET.to_string()),
        _ => None,
    };
    return retval;
}

pub fn getModalityFromProcedureDesc(desc: String) -> Option<String> {
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

pub fn getLocationSiteMapping(location: &String) -> Option<String> {
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
