use std::error::Error;

use chrono::NaiveDate;

fn main() -> Result<(), Box<dyn Error>> {
    print!("{}[2J", 27 as char);
    for _ in 1..10 {
        println!();
    }
    println!("Starting.");

    let facility_start:NaiveDate=NaiveDate::from_ymd_opt(2024, 1, 2).expect("Should be a valid date.");
    let facility_end:NaiveDate=NaiveDate::from_ymd_opt(2024, 3, 24).expect("Should be a valid date.");

    let rotation_start:NaiveDate=NaiveDate::from_ymd_opt(2024, 1, 6).expect("Should be a valid date.");
    let rotation_end:NaiveDate=NaiveDate::from_ymd_opt(2024, 3, 24).expect("Should be a valid date.");

    let mut common = rotation_tool::build_main_common()?;

    let rotation_analysis: bool = true;
    if rotation_analysis {
        common.analyze_rotations()?;
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
