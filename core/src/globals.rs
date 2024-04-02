pub mod file_names {
    pub(crate) const MAIN_DATA_FILE: &str = "./data/2024 to today.csv";
    pub(crate) const BVU_DATA_FILE: &str = "./data/BVU.csv";
    pub(crate) const BVU_UPDATE_FILE: &str = "./data/BVU_modified.csv";
    pub(crate) const CATEGORIES_LOCATION_FILE: &str = "./categories/Categories_Location.csv";
    pub(crate) const CATEGORIES_EXAM_FILE: &str = "./categories/Categories_Exam.csv";
    pub(crate) const EXAMPLE_ROTATION_DESCRIPTIONS: &str = "./rotations/Example.yml";
    pub(crate) const SOURCE_CACHE: &str = "./cache/ProcessedSource.yml";
    pub(crate) const COVERAGE_AUDIT_OUT: &str = "./err/Coverage_Audit.tsv";
    pub(crate) const COVERAGE_AUDIT_NOWORK_OUT: &str = "./err/Coverage_Audit (no work).tsv";
    pub(crate) const COVERAGE_ANALYSIS_OUT: &str = "./output/WeekAnalysis";
    pub(crate) const VOLUME_BY_DATE_ROTATION: &str = "volume_by_date_and_rotation";
}

pub mod main_headers {
    pub(crate) enum PertinentHeaders {
        Accession,
        ProcedureCode,
        Exam,
        Location,
        ScheduledDatetime,
        Rvu,
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
pub(crate) const MSK_WE_AH0C: &str = "MSK Weekend AH0C";
pub(crate) const NEURO: &str = "Neuro";

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
    "Non-Radiology",
];

const INPATIENT: &str = "Inpatient";
pub const OUTPATIENT: &str = "Outpatient";
const ED: &str = "ED";

pub const CONTEXTS: &[&str] = &[INPATIENT, OUTPATIENT, ED, "Wet Read"];

//modalities
const XR: &str = "XR";
const MG: &str = "MG";
const ANG: &str = "ANG";
const US: &str = "US";
const PET: &str = "PET";

pub const MODALITIES: &[&str] = &[
    XR, "CT", US, "MR", "NM", PET, "DEXA", "RF", MG, "XA", "CVUS", ANG, "CLINIC",
];

pub fn map_site_to_context(site: &str) -> Option<String> {
    match site {
        SH => Some(OUTPATIENT.to_string()),
        SC => Some(OUTPATIENT.to_string()),
        WB => Some(OUTPATIENT.to_string()),
        _ => None,
    }
}

pub fn get_modality_alias(modality: &String) -> Option<String> {
    let retval = match modality.as_str() {
        "MAM" => Some(MG.to_string()),
        "CR" => Some(XR.to_string()),
        "PT" => Some(PET.to_string()),
        _ => None,
    };
    retval
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
