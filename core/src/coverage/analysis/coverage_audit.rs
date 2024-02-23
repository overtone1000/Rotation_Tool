use std::{collections::{hash_map::Entry, HashMap}, error::Error, io::Write};

use crate::{analysis::analysis_datum::AnalysisDatum, coverage::{coordinate::CoverageCoordinates, coverage_and_work_day::CoverageAndWorkDay, malformed_coverage::CoverageError, units::Coverage, work_coverage_map::CoverageMap}, globals::ALL_DAYS, rotations::rotation_error::RotationManifestParseError};

impl CoverageMap
{
pub fn audit(&mut self) -> HashMap<CoverageCoordinates, CoverageError> {
    let mut retval: HashMap<CoverageCoordinates, CoverageError> = HashMap::new();

    //let testcoords=testcoords();

    let func =
        |coords: &CoverageCoordinates, coverage_and_workday: &mut CoverageAndWorkDay| {
            let errs = coverage_and_workday.audit_coverage();

            retval.insert(coords.to_owned(), errs);
        };

    self.foreach(func);

    retval
}


pub fn audit_to_stream<T: Write>(&mut self, primary_error_writer: &mut T, work_gap_writer: &mut T) -> Result<(), Box<dyn Error>> {
    let audit_result = self.audit();

    let mut sorted_keys: Vec<&CoverageCoordinates> = audit_result.keys().collect();
    sorted_keys.sort();

    let mut no_errs=true;

    let header = "Site \u{0009} Exam \u{0009} Context \u{0009} Day of Week \n";
    
    primary_error_writer.write(header.as_bytes())?;
    work_gap_writer.write(header.as_bytes())?;

    for coords in sorted_keys {
        let coordstr = format!("{} \u{0009} {} \u{0009} {} \u{0009} {} \u{0009}",
            coords.site,
            coords.subspecialty,
            coords.context,
            //coords.modality,
            coords.weekday
        );
        let errs = audit_result.get(coords).expect("Should be a key");
        match errs {
            CoverageError::NoCoverage(rvus) => {
                no_errs=false;
                writeln!(
                    primary_error_writer,
                    "{} No coverage ({} rvus)",
                    coordstr,
                    rvus
                )?;
            }
            CoverageError::MalformedCoverage(errs) => {
                if !errs.gaps.is_empty() {
                    for (rotation1, rotation2, desc, rvus) in &errs.gaps {
                        no_errs=false;
                        writeln!(
                            primary_error_writer,
                            "{} Coverage gap: {}-{} {} ({} rvus)",
                            coordstr,
                            rotation1,
                            rotation2,
                            desc,
                            rvus
                        )?;
                    }
                }
                if !errs.overlaps.is_empty() {
                    for overlap in &errs.overlaps {
                        no_errs=false;
                        writeln!(
                            primary_error_writer,
                            "{} Coverage overlap: {}",
                            coordstr,
                            overlap
                        )?;
                    }
                }
                match errs.incorrect_fraction {
                    Some(x) => {
                        no_errs=false;
                        writeln!(
                            primary_error_writer,
                            "{} Incorrect fraction: {}",
                            coordstr,
                            x
                        )?;
                    }
                    None => {
                        ()
                    },
                }
                if errs.no_work {
                    writeln!(
                        work_gap_writer,
                        "{} No work",
                        coordstr
                    )?;
                }
            }
        };
    }

    if no_errs
    {
        writeln!(primary_error_writer, "No errors detected.")?;
        Ok(())
    }
    else {
        RotationManifestParseError::generate_boxed(0, "Audit returned errors.".to_string())
    }
}
}