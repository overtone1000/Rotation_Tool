use std::{collections::{HashMap, HashSet}, fs::File};

use csv::{Reader, StringRecordsIter};
use serde::Serialize;
use statrs::function;

use crate::serialization::output::JSONFileOut;

pub trait Table<T>
{
    fn get_file_path(&self)->&str;

    fn build_from_headers_and_row(header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<T, Box<dyn std::error::Error>>;
    
    fn get_from_row_with_header(header:&str, header_map:&HashMap<String,usize>, row:&Vec<String>)->String
    {
        let index=*(header_map.get(header).expect("Should have this header."));
        row.get(index).expect("Should have this member").to_string()
    }

    fn write(filename:&str, headers:&[String], entries:Vec<Vec<String>>)->Result<(),Box<dyn std::error::Error>>
    {
        let mut writer = csv::WriterBuilder::new()
            .delimiter(b',')
            .quote(b'"')
            .has_headers(true)
            .from_path(filename)?;
        
        for entry in entries
        {
            writer.write_record(entry);
        }

        writer.flush();

        Ok(())
    }

    fn iter(&self)->TableIter<'_,T>
    {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .quote(b'"')
            .has_headers(true)
            .from_path(self.get_file_path()).expect(format!("Couldn't parse CSV file {}",self.get_file_path()).as_str());

        let mut headers: Vec<String>=Vec::new();
        let mut labelmap: HashMap<String, usize>=HashMap::new();

        let mut n: usize = 0;
        match rdr.headers()
        {
            Ok(header_records) => {
                for header in header_records {
                    headers.push(header.to_string());
                    labelmap.insert(header.to_string(), n);
                    n += 1;
                }
            },
            Err(x) => panic!("Couldn't parse headers. {:?}",x),
        }
        
        
        let bfunc =& |header_map:&HashMap<String,usize>, row:&Vec<String>|{Self::build_from_headers_and_row(header_map, row)};

        TableIter { 
            build_function: bfunc,
            iter: rdr.records(),
            headers: headers, 
            labelmap: labelmap
        }
    }
}

struct TableIter<'a,T>
{
    build_function:&'a dyn Fn(&HashMap<String,usize>,&Vec<String>)->Result<T, Box<dyn std::error::Error>>,
    iter:StringRecordsIter<'a,File>,
    headers:Vec<String>,
    labelmap:HashMap<String,usize>
}

impl<'a,T> Iterator for TableIter<'a,T>
{
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {

        match self.iter.next()
        {
            Some(record) => {
                match record
                {
                    Ok(record) => {
                        let mut row: Vec<String> = Vec::new();
                        for cell in record.iter() {
                            row.push(cell.to_string());
                        }

                        if row.len() != self.labelmap.keys().len() {
                            let mut message: String = "Malformed data. Header length is ".to_string();
                            message += &(self.headers.len().to_string());
                            message += " but row ";
                            message += format!("{:?}",row).as_str();
                            message += " contains ";
                            message += &(row.len().to_string());
                            message += " items.";
                            eprintln!("{}",message);
                        }

                        Some((self.build_function)(&self.labelmap,&row).expect("Malformed table?"))
                    },
                    Err(e) => {
                        eprintln!("{}",e.to_string());
                        None
                    },
                }
            },
            None => None,
        }
    }
}