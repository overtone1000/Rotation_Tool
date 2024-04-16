use std::{collections::{HashMap, HashSet}, error::Error, fs::File, str::FromStr};

use chrono::NaiveDateTime;
use csv::{Reader, StringRecordsIntoIter, StringRecordsIter};
use serde::Serialize;
use statrs::function;

use crate::serialization::output::JSONFileOut;

pub trait Table<T>
{
    fn get_file_path(&self)->&str;

    fn build_from_headers_and_row(header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<T, Box<dyn std::error::Error>>;
    
    fn get_from_row_with_header(header:&str, header_map:&HashMap<String,usize>, row:&Vec<String>)->String
    {
        let index=*(header_map.get(header).expect(format!("Header {} not found. Header map: {:?}",header,header_map).as_str()));
        row.get(index).expect("Should have this member").to_string()
    }

    fn get_as_date(header:&str, header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<NaiveDateTime,Box<dyn std::error::Error>>
    {
        let time_string=Self::get_from_row_with_header(header, header_map, row);

        //let test2 = chrono::NaiveDate::from_ymd_opt(2023,11,9).unwrap().and_hms_opt(8,53,0).unwrap();
        //let test2str=test2.format("%m/%d/%y %H:%M").to_string();
        //let test = NaiveDateTime::parse_from_str("11/09/2023 08:53", "%m/%d/%y %H:%M").expect("Huh?");

        match NaiveDateTime::parse_from_str(&time_string, "%m/%d/%Y %H:%M") //expects format like 11/09/2023 8:53
        {
            Ok(x)=>Ok(x),
            Err(e)=>{
                eprintln!("Bad date time {}",time_string);
                return Err(Box::new(e));
            }
        }
    }

    fn parse<U>(header:&str, header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<U,Box<dyn std::error::Error>>
    where U: FromStr
    {
        let val=Self::get_from_row_with_header(header, header_map, row);
        let str=match val.as_str()
        {
            "NULL"=>"0",
            x=>x
        };
        
        match str.parse()
        {
            Ok(x)=>Ok(x),
            Err(e)=>{
                eprintln!("Bad parse {}",str);
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData,"Bad parse")));
            }
        }
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

    fn iter<'a>(&'a self)->TableIter<'a,T>
    {
        println!("Accessing table {}",self.get_file_path());
        let mut rdr: Reader<File> = csv::ReaderBuilder::new()
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
            iter:rdr.into_records(),
            headers: headers, 
            labelmap: labelmap
        }
    }
}

pub struct TableIter<'a,T>
{
    build_function:&'a dyn Fn(&HashMap<String,usize>,&Vec<String>)->Result<T, Box<dyn std::error::Error>>,
    iter:StringRecordsIntoIter<File>,
    headers:Vec<String>,
    labelmap:HashMap<String,usize>
}

impl<'a,T> Iterator for TableIter<'a,T>
{
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item>{

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
                        Some(
                            match (self.build_function)(&self.labelmap,&row)
                            {
                                Ok(x)=>x,
                                Err(e)=>{
                                    panic!("{}",e.to_string());
                                }
                            }
                        )
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