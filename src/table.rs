use std::collections::HashSet;
use std::fs;
use std::ops::Range;
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
        
        let mut retval=Table { 
            headers: Vec::new(), 
            data: Vec::new(),
            labelmap: HashMap::new()
        };

        let mut n:usize=0;
        for header in rdr.headers()?.iter() {
            retval.headers.push(header.to_string());
            retval.labelmap.insert(header.to_string(),n);
            n+=1;
        }

        for record in rdr.records() {
            let mut row:Vec<String>=Vec::new();
            for cell in record?.iter()
            {
                row.push(cell.to_string());
            }
            if(row.len()!=retval.headers.len())
            {
                let mut message:String = "Malformed data. Header length is ".to_string();
                message+=&(retval.headers.len().to_string());
                message+=" but row ";
                message+=&(retval.data.len().to_string());
                message+=" contains ";
                message+=&(row.len().to_string());
                message+=" items.";
                return Err(Error::new(ErrorKind::InvalidData, message));
            }
            retval.data.push(row);
        }


        Ok(retval)
    }

    fn getHeaderColumnIndex(&self, header_label:&String)->Option<&usize>
    {
        self.labelmap.get(header_label)
    }

    pub fn getVal(&self, header_label:&String, row:usize)->Option<String>
    {
        let index=self.getHeaderColumnIndex(header_label)?.to_owned();
        let datarow=self.data.get(row)?.to_owned();
        let cell=datarow.get(index);

        match cell{
            None=>None,
            Some(val)=>Some(val.to_owned())
        }
    }

    pub fn rowIndices(&self)->Range<usize>
    {
        0..self.data.len()
    }

    pub fn getKeyedColumnValueMap(&self, key_header_label:&String, adjoined_header_labels:&[String])->HashMap<String,Vec<String>>
    {
        let mut retval:HashMap<String,Vec<String>>=HashMap::new();
        for row_i in self.rowIndices()
        {
            let key_value=self.getVal(key_header_label, row_i);
            match key_value
            {
                None=>{},
                Some(key_value)=>{
                    let mut adjoined:Vec<String>=Vec::new();
                    for adjoined_header_label in adjoined_header_labels
                    {
                        let adjoined_value=self.getVal(adjoined_header_label, row_i);
                        match adjoined_value{
                            None=>{adjoined.push("".to_string());}
                            Some(av)=>{adjoined.push(av.to_owned());}
                        }
                    }
                    retval.insert(key_value, adjoined);
                }
            }
        }

        retval
    }
}
