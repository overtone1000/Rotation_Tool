use std::fs;
use std::vec::Vec;
use std::io::Error;
use std::io::ErrorKind;
use std::collections::HashMap;

pub struct Table
{
    headers:Vec<String>,
    data:Vec<Vec<String>>,
    labelmap:HashMap<String,usize>
}

impl Table
{
    pub fn create(file_path:&str)->Result<Table,std::io::Error>
    {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .quote(b'"')
            .has_headers(true)
            .from_path(file_path)?;
        

        let mut headers:Vec<String>=Vec::new();
        let mut data:Vec<Vec<String>>=Vec::new();
        let mut labelmap:HashMap<String,usize>=HashMap::new();

        println!("Headers");
        for header in rdr.headers() {
            headers.push(header.as_slice().to_string());
        }

        println!("Rows");
        for record in rdr.records() {
            let mut row:Vec<String>=Vec::new();
            for cell in record?.iter()
            {
                row.push(cell.to_string());
            }
            if(row.len()!=headers.len())
            {
                let mut message:String = "Malformed data. Header length is ".to_string();
                message+=&(headers.len().to_string());
                message+=" but row ";
                message+=&(data.len().to_string());
                message+=" contains ";
                message+=&(row.len().to_string());
                message+=" items.";
                return Err(Error::new(ErrorKind::InvalidData, message));
            }
            data.push(row);
        }

        print!("Stop!");

        /*
        let file=fs::read_to_string(file_path.to_string())?;
        let rows=file.lines();

        for row in rows
        {
            let cells = row.split(",");
            let mut newrow:Vec<String>=Vec::new();

            for cell in cells
            {
                newrow.push(cell.to_string());
            }

            match &headers
            {
                None => {
                    let mut n:usize=0;
                    for header in &newrow
                    {
                        labelmap.insert(header.to_owned(), n);
                        n+=1;
                    }
                    headers=Some(newrow);
                },
                Some(x) => {
                    if(newrow.len()!=x.len())
                    {
                        
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
                        data:data,
                        labelmap:labelmap
                    }
                )
            }
        }
        */

        return Err(Error::new(ErrorKind::InvalidData,"No headers found."));
    }

    pub fn getHeaderColumnIndex(&self, header_label:String)->Option<&usize>
    {
        return self.labelmap.get(&header_label);
    }
}
