use std::error::Error;

use chrono::NaiveDate;
use rotation_tool::{print_averages_by_modality_and_day, MainCommon};

fn main() -> Result<(), Box<dyn Error>> {
    print!("{}[2J", 27 as char);
    for _ in 1..10 {
        println!();
    }
    println!("Starting.");

    let facility_start=&NaiveDate::from_ymd_opt(2024, 1, 2).expect("Should be a valid date.");
    let facility_end=&NaiveDate::from_ymd_opt(2024, 3, 24).expect("Should be a valid date.");

    let rotation_start=&NaiveDate::from_ymd_opt(2024, 1, 6).expect("Should be a valid date.");
    let rotation_end=&NaiveDate::from_ymd_opt(2024, 3, 24).expect("Should be a valid date.");

    let mut common = rotation_tool::build_main_common()?;

    let print_averages:bool=true;
    if print_averages {
        print_averages_by_modality_and_day(&common.coverage_tree, rotation_start, rotation_end);
    }

    let rotation_analysis: bool = true;
    if rotation_analysis {
        MainCommon::analyze_rotations(&mut common)?;
    }

    let generate_frontend_statics: bool = true;
    if generate_frontend_statics {
        common.generate_frontend_statics(
            facility_start,
            facility_end,
            rotation_start,
            rotation_end
        )?;
    }

    let perform_detailed_analysis: bool = false;
    if perform_detailed_analysis {
        common.perform_detailed_analysis()?;
    }

    println!("Finished.");
    Ok(())
}
