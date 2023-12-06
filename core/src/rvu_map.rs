use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::Write,
};

use chrono::{NaiveDate, NaiveDateTime, Timelike};

use crate::{
    categorization::{build_salem_bvumap, build_salem_rvumap},
    constraints::ConstraintSet,
    dates::BUSINESS_DAYS_PER_YEAR,
    error::RotationToolError,
    globals::{file_names, main_headers, tpc_headers, CONTEXTS, MODALITIES, SITES, SUBSPECIALTIES},
    processed_source::ProcessedSource,
    rvu_map,
};

struct MapEntry {
    rvus: f64,
}

impl MapEntry {
    fn add_rvus(&mut self, rvus: f64) {
        self.rvus += rvus;
    }

    fn get_rvus(&self) -> f64 {
        self.rvus.to_owned()
    }

    fn set_rvus(&mut self, rvu: f64) {
        self.rvus = rvu;
    }
}

#[derive(Default)]
pub struct MapCoords {
    site: String,
    subspecialty: String,
    context: String,
    modality: String,
    time_row: usize,
}

impl MapCoords {
    fn validate(s: String, list: &[&str]) -> bool {
        for member in list {
            if *member == s {
                return true;
            }
        }
        false
    }
    pub fn validate_site(&self) -> bool {
        let retval = MapCoords::validate(self.site.to_owned(), SITES);
        if !retval {
            eprintln!("Invalid site {}", self.site);
        }
        retval
    }
    pub fn validate_subspecialty(&self) -> bool {
        let retval = MapCoords::validate(self.subspecialty.to_owned(), SUBSPECIALTIES);
        if !retval {
            eprintln!("Invalid subspecialty {}", self.subspecialty);
        }
        retval
    }
    pub fn validate_context(&self) -> bool {
        let retval = MapCoords::validate(self.context.to_owned(), CONTEXTS);
        if !retval {
            eprintln!("Invalid context {}", self.context);
        }
        retval
    }
    pub fn validate_modality(&self) -> bool {
        let retval = MapCoords::validate(self.modality.to_owned(), MODALITIES);
        if !retval {
            eprintln!("Invalid modality {}", self.modality);
        }
        retval
    }

    pub fn get_subspecialty(&self) -> &String {
        &self.subspecialty
    }
    pub fn get_context(&self) -> &String {
        &self.context
    }
    pub fn get_site(&self) -> &String {
        &self.site
    }
}

pub struct RVUMap {
    //site, subspecialty, context, modality, time_row
    map: HashMap<
        String,
        HashMap<String, HashMap<String, HashMap<String, HashMap<usize, MapEntry>>>>,
    >,
}

impl RVUMap {
    fn new() -> RVUMap {
        

        RVUMap {
            map: HashMap::new(),
        }
    }

    fn add_rvus(&mut self, coords: &MapCoords, rvus: f64) -> Result<String, String> {
        if !coords.validate_site() {
            return Err("Invalid site.".to_string());
        }
        if !self.map.contains_key(&coords.site) {
            let map = HashMap::new();
            self.map.insert(coords.site.to_owned(), map);
        }
        let sub_map = self.map.get_mut(&coords.site).expect("Immediate get");

        if !coords.validate_subspecialty() {
            return Err("Invalid subspecialty.".to_string());
        }
        if !sub_map.contains_key(&coords.subspecialty) {
            let map = HashMap::new();
            sub_map.insert(coords.subspecialty.to_owned(), map);
        }
        let con_map = sub_map
            .get_mut(&coords.subspecialty)
            .expect("Immediate get");

        if !coords.validate_context() {
            return Err("Invalid context.".to_string());
        }
        if !con_map.contains_key(&coords.context) {
            let map = HashMap::new();
            con_map.insert(coords.context.to_owned(), map);
        }
        let mod_map = con_map.get_mut(&coords.context).expect("Immediate get");

        if !coords.validate_modality() {
            return Err("Invalid modality.".to_string());
        }
        if !mod_map.contains_key(&coords.modality) {
            let map = HashMap::new();
            mod_map.insert(coords.modality.to_owned(), map);
        }
        let time_map = mod_map.get_mut(&coords.modality).expect("Immediate get");

        time_map.entry(coords.time_row).or_insert_with(|| {
            let map_entry: MapEntry = MapEntry { rvus: 0.0 };
            map_entry
        });
        let me = time_map.get_mut(&coords.time_row).expect("Immediate get");
        me.add_rvus(rvus);
        Ok("good".to_string())
    }

    pub fn to_json(&self, constraints: &Option<ConstraintSet<MapCoords>>) -> Result<String, String> {
        let mut topnode = json::JsonValue::new_object();
        if self.map.keys().len() > 0 {
            for site in self.map.keys() {
                let sub_map = self.map.get(site).expect("No submap");
                if sub_map.keys().len() > 0 {
                    let mut sitenode: json::JsonValue = json::JsonValue::new_object();
                    for subspecialty in sub_map.keys() {
                        let con_map = sub_map.get(subspecialty).expect("No conmap");
                        if con_map.keys().len() > 0 {
                            let mut subspecialtynode = json::JsonValue::new_object();
                            for context in con_map.keys() {
                                let mod_map = con_map.get(context).expect("No modmap");
                                if mod_map.keys().len() > 0 {
                                    let mut contextnode = json::JsonValue::new_object();
                                    for modality in mod_map.keys() {
                                        let time_map = mod_map.get(modality).expect("No time map");
                                        if time_map.keys().len() > 0 {
                                            let mut modalitynode = json::JsonValue::new_object();
                                            for time_row in time_map.keys() {
                                                let coords = MapCoords {
                                                    site: site.to_owned(),
                                                    subspecialty: subspecialty.to_owned(),
                                                    context: context.to_owned(),
                                                    modality: modality.to_owned(),
                                                    time_row: time_row.to_owned(),
                                                };

                                                let include = match &constraints {
                                                    Some(constraints) => {
                                                        constraints.include(&coords)
                                                    }
                                                    None => true,
                                                };

                                                if include {
                                                    let me = time_map
                                                        .get(time_row)
                                                        .expect("No map entry");
                                                    modalitynode[time_row.to_string()] =
                                                        me.rvus.into();
                                                }
                                            }
                                            contextnode[modality] = modalitynode;
                                        }
                                    }
                                    subspecialtynode[context] = contextnode;
                                }
                            }
                            sitenode[subspecialty] = subspecialtynode;
                        }
                    }
                    topnode[site] = sitenode;
                }
            }
        }

        Ok(topnode.dump())
    }

    pub fn to_file(
        &self,
        constraints: &Option<ConstraintSet<MapCoords>>,
        filename: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut mapoutfile = File::create(filename)?;
        let mapstr = self.to_json(constraints)?;
        let bytes = mapstr.as_bytes();

        match mapoutfile.write_all(bytes) {
            Ok(_) => Ok(()),
            Err(e) => {
                Err(Box::new(crate::error::RotationToolError::new(e.to_string())))
            }
        }
    }

    pub fn total_average_rvus(&self) -> f64 {
        self.slice_average_rvus(None)
    }

    pub fn slice_average_rvus(&self, constraints: Option<ConstraintSet<MapCoords>>) -> f64 {
        //site, subspecialty, context, modality, time_row
        let mut retval: f64 = 0.0;
        for (site, m1) in &self.map {
            for (subspecialty, m2) in m1 {
                for (context, m3) in m2 {
                    for (modality, m4) in m3 {
                        for (time_row, me) in m4 {
                            let coords = MapCoords {
                                site: site.to_owned(),
                                subspecialty: subspecialty.to_owned(),
                                context: context.to_owned(),
                                modality: modality.to_owned(),
                                time_row: time_row.to_owned(),
                            };

                            let include = match &constraints {
                                Some(constraints) => constraints.include(&coords),
                                None => true,
                            };

                            if include {
                                if me.rvus.is_infinite() {
                                    eprintln!("Infinite RVUs!");
                                }
                                retval += me.rvus;
                                if retval.is_infinite() {
                                    eprintln!("Infinite retval!");
                                }
                            }
                        }
                    }
                }
            }
        }
        retval
    }
}

pub fn create_map(
    source: &ProcessedSource,
    exam_rvu_map: &HashMap<String, f64>,
    date_constraints: &ConstraintSet<'_, NaiveDateTime>,
) -> Result<RVUMap, String> {
    let mut rvumap = RVUMap::new();

    let mut modality_map: HashMap<String, String> = HashMap::new();

    let mut included_dates: HashSet<NaiveDate> = HashSet::new();

    for row_i in source.main_data_table.row_indices() {
        let datetimestring = source.main_data_table.get_val(
            &main_headers::PertinentHeaders::ScheduledDatetime.get_label(),
            &row_i,
        )?;

        let datetime = match NaiveDateTime::parse_from_str(&datetimestring, "%m/%d/%y %H:%M") {
            Ok(x) => x,
            Err(_x) => {
                return Err(format!("Couldn't parse date {}", datetimestring));
            }
        };

        let location = source.main_data_table.get_val(
            &main_headers::PertinentHeaders::Location.get_label(),
            &row_i,
        )?;
        let exam_code = source.main_data_table.get_val(
            &main_headers::PertinentHeaders::ProcedureCode.get_label(),
            &row_i,
        )?;

        //Build coords and populate maps with this row.
        let mut coords = MapCoords::default();
        {
            coords.time_row = crate::time::get_time_row_index(datetime.hour(), datetime.minute());

            //Get subspecialty from exam code
            coords.subspecialty = match source.exam_to_subspecialty_map.get(&exam_code) {
                Some(x) => x.to_string(),
                None => {
                    return Err(format!(
                        "Invalid exam_code {} in exam_to_subspeciality_map",
                        exam_code
                    ));
                }
            };

            //Try site. If not valid, go by location.
            let mut selected_site: Option<String> = None;
            let listed_site = source.main_data_table.get_val(
                &main_headers::PertinentHeaders::Accession.get_label(),
                &row_i,
            )?;
            for site in SITES {
                if (listed_site[0..site.len()]).to_ascii_uppercase()
                    == site.to_string().to_ascii_uppercase()
                {
                    selected_site = Some(site.to_string());
                    break;
                }
            }
            if selected_site.is_none() {
                selected_site = crate::globals::get_location_site_mapping(&location);
            }
            coords.site = match selected_site {
                Some(x) => x,
                None => {
                    return Err(format!("Could not determine site for row {}", row_i));
                }
            };

            //Try context. If not valid, go by site map.
            coords.context = match source.location_to_context_map.get(&location) {
                Some(x) => x.to_string(),
                None => match crate::globals::get_location_site_mapping(&location) {
                    Some(x) => x,
                    None => {
                        return Err(format!(
                            "Could not determine context for location {}",
                            location
                        ));
                    }
                },
            };

            //Get modality, but check for aliases
            let listed_modality = source.main_data_table.get_val(
                &main_headers::PertinentHeaders::Modality.get_label(),
                &row_i,
            )?;
            let mut selected_modality: Option<String> = None;
            for modality in MODALITIES {
                if *modality == listed_modality {
                    selected_modality = Some(modality.to_string());
                    break;
                }
            }
            match selected_modality {
                None => {
                    selected_modality = crate::globals::get_modality_alias(&listed_modality);
                }
                _ => {}
            }
            match selected_modality {
                None => {
                    selected_modality = crate::globals::get_modality_from_procedure_desc(
                        source
                            .main_data_table
                            .get_val(&main_headers::PertinentHeaders::Exam.get_label(), &row_i)?,
                    )
                }
                _ => {}
            }
            coords.modality = match selected_modality {
                Some(x) => x,
                None => {
                    return Err(format!("Could not determine modality for row {}", row_i));
                }
            };
            if !modality_map.contains_key(&exam_code) {
                modality_map.insert(exam_code.to_owned(), coords.modality.to_owned());
            }
        }

        //Check if this date should be included in RVU totals. If so, add rvus.
        if date_constraints.include(&datetime) {
            included_dates.insert(NaiveDate::from(datetime));

            //let rvus_str = main_data_table.getVal(&main_headers::pertinent_headers::rvu.getLabel(), &row_i)?;
            let rvus = match exam_rvu_map.get(&exam_code) {
                Some(&x) => x,
                None => {
                    return Err(format!("Coudn't find exam code {}", exam_code));
                }
            };

            rvumap.add_rvus(&coords, rvus)?;
        }
    }

    let days: f64 = included_dates.len() as f64;

    if days == 0.0 {
        eprintln!("Zero days!!");
    }

    //Divide by number of days worth of data to get rvu/day
    for site in rvumap.map.iter_mut() {
        let sub_map = site.1;
        for subspecialty in sub_map.iter_mut() {
            let con_map = subspecialty.1;
            for context in con_map.iter_mut() {
                let mod_map = context.1;
                for modality in mod_map.iter_mut() {
                    let time_map = modality.1;
                    for time_row in time_map.iter_mut() {
                        let me = time_row.1;
                        me.set_rvus(me.rvus / days);
                    }
                }
            }
        }
    }

    //Add TPC, which doesn't go by number of dates
    let weights = crate::time::get_time_row_normal_dist_weights();
    for row_i in source.tpc_data_table.row_indices() {
        let exam_code = source.tpc_data_table.get_val(
            &tpc_headers::PertinentHeaders::ExamCode.get_label(),
            &row_i,
        )?;
        let number_str = source.tpc_data_table.get_val(
            &tpc_headers::PertinentHeaders::NumberIn2022.get_label(),
            &row_i,
        )?;

        let number = match number_str.parse::<f64>() {
            Ok(val) => val,
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        };

        let number_per_business_day = number / BUSINESS_DAYS_PER_YEAR;
        let rvus_per_exam = match exam_rvu_map.get(&exam_code) {
            None => {
                return Err(format!("Bad exam code {}", exam_code));
            }
            Some(val) => val.to_owned(),
        };

        let rvus_per_business_day = number_per_business_day * rvus_per_exam;

        let mut coords = MapCoords::default();
        coords.site = crate::globals::TPC.to_string();
        coords.subspecialty = match source.exam_to_subspecialty_map.get(&exam_code) {
            None => {
                return Err(format!("Bad exam code {}", exam_code));
            }
            Some(val) => val.to_owned(),
        };
        coords.context = crate::globals::OUTPATIENT.to_string();
        coords.modality = match modality_map.get(&exam_code) {
            None => {
                return Err(format!("Bad exam code {}", exam_code));
            }
            Some(val) => val.to_owned(),
        };

        for key in weights.keys() {
            coords.time_row = *key;
            let rvu = rvus_per_business_day * (*weights.get(key).expect("Expected")) as f64;
            rvumap.add_rvus(&coords, rvu);
        }
    }

    Ok(rvumap)
}

pub fn build_maps(
    date_constraints: &ConstraintSet<NaiveDateTime>,
    classification_constraints: Option<ConstraintSet<MapCoords>>,
) -> Result<(), Box<dyn Error>> {
    let source = ProcessedSource::build()?;

    //Create the conventional RVU map
    {
        let rvu_map = build_salem_rvumap(&source.main_data_table)?;
        let map = match rvu_map::create_map(&source, &rvu_map, date_constraints) {
            Ok(x) => x,
            Err(e) => {
                let err = RotationToolError::new(e);
                return Err(Box::new(err));
            }
        };

        match map.to_file(&classification_constraints, file_names::OUT_FILE) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }
    }

    //Create BVU map
    {
        let bvu_map: HashMap<String, f64> = build_salem_bvumap(&source.bvu_data_table)?;
        let map = match rvu_map::create_map(&source, &bvu_map, date_constraints) {
            Ok(x) => x,
            Err(e) => {
                let err = RotationToolError::new(e);
                return Err(Box::new(err));
            }
        };

        match map.to_file(&classification_constraints, file_names::BVU_OUT_FILE) {
            Ok(_) => {}
            Err(e) => {
                return Err(e);
            }
        }
    }

    println!("Finished.");
    Ok(())
}
