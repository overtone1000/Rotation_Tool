use std::error::Error;

use chrono::NaiveDate;

fn main() -> Result<(), Box<dyn Error>> {
    print!("{}[2J", 27 as char);
    for _ in 1..10 {
        println!();
    }
    println!("Starting.");

    let mut common = rotation_tool::build_main_common()?;

    let start:NaiveDate=NaiveDate::from_ymd_opt(2024, 1, 13).expect("Should be a valid date.");
    let end:NaiveDate=NaiveDate::from_ymd_opt(2024, 3, 17).expect("Should be a valid date.");
    common.coverage_tree.prune_by_rotation_date(start,end);

    let rotation_analysis: bool = true;
    if rotation_analysis {
        rotation_tool::analyze_rotations(&mut common)?;
    }

    let generate_frontend_statics: bool = true;
    if generate_frontend_statics {
        rotation_tool::generate_frontend_statics(&mut common)?;
    }

    let perform_detailed_analysis: bool = false;
    if perform_detailed_analysis {
        rotation_tool::perform_detailed_analysis(&mut common)?;
    }

    println!("Finished.");
    Ok(())
}
