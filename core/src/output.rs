use std::{error::Error, fs::File, io::BufWriter};

use serde::Serialize;

pub trait JSONable:Serialize
{
    fn to_json(&self, filename:&str) -> Result<(), Box<dyn Error>> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
}