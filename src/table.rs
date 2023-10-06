use std::fs;
use std::vec::Vec;
use std::io::Error;
use std::io::ErrorKind;
use std::collections::HashMap;

pub struct Table
{
    headers:Vec<String>,
    data:Vec<Vec<String>>,
    headerlabelmap:HashMap<String,usize>
}

impl Table
{
    pub fn create(file_path:String)->Result<Table,std::io::Error>
    {
        let file=fs::read_to_string(file_path)?;
        let rows=file.lines();

        let mut headers:Option<Vec<String>>=None;
        let mut data:Vec<Vec<String>>=Vec::new();

        populate hasmap

        //Main data rows
        for row in rows
        {
            let cells = row.rsplit("\t");
            let mut newrow:Vec<String>=Vec::new();

            for cell in cells
            {
                newrow.push(cell.to_string());

            }

            match &headers
            {
                None => headers=Some(newrow),
                Some(x) => {
                    if(newrow.len()!=x.len())
                    {
                        return Err(Error::new(ErrorKind::InvalidData,"Malformed data."));
                    }
                    data.push(newrow.to_owned())
                },
            }
        }

        match headers
        {
            None=>Err(Error::new(ErrorKind::InvalidData,"No headers found.")),
            Some(x)=>{
                Ok(
                    Table{
                        headers:x.to_owned(),
                        data:data
                    }
                )
            }
        }
    }

    pub fn getHeaderColumnIndex(&self, header_label:String)->Option<i32>
    {
        let mut n:i32=0;
        for header in self.headers
        {
            if(header==header_label)
            {
                return Some(n);
            }
            else {
                n+=1;
            }
        }

        return None;
    }
}
