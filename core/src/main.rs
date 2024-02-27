use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    print!("{}[2J", 27 as char);
    for _ in 1..10 {
        println!();
    }
    println!("Starting.");

    let mut common = rotation_tool::build_main_common()?;

    let rebuild_source: bool = true;

    if rebuild_source {
        rotation_tool::cache_source()?;
    }

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
