use std::collections::HashMap;

use std::io::Error;
use std::io::ErrorKind;
use std::ops::Range;
use std::vec::Vec;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Table {
    headers: Vec<String>,
    data: Vec<Vec<String>>,
    labelmap: HashMap<String, usize>,
}

impl Table {
    pub fn create(file_path: &str) -> Result<Table, std::io::Error> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .quote(b'"')
            .has_headers(true)
            .from_path(file_path)?;

        let mut retval = Table {
            headers: Vec::new(),
            data: Vec::new(),
            labelmap: HashMap::new(),
        };

        let mut n: usize = 0;
        for header in rdr.headers()?.iter() {
            retval.headers.push(header.to_string());
            retval.labelmap.insert(header.to_string(), n);
            n += 1;
        }

        for record in rdr.records() {
            let mut row: Vec<String> = Vec::new();
            for cell in record?.iter() {
                row.push(cell.to_string());
            }
            if row.len() != retval.headers.len() {
                let mut message: String = "Malformed data. Header length is ".to_string();
                message += &(retval.headers.len().to_string());
                message += " but row ";
                message += &(retval.data.len().to_string());
                message += " contains ";
                message += &(row.len().to_string());
                message += " items.";
                return Err(Error::new(ErrorKind::InvalidData, message));
            }
            retval.data.push(row);
        }

        Ok(retval)
    }

    pub fn structural_clone(&self) -> Table
    {
        Table {
            headers:self.headers.clone(),
            labelmap:self.labelmap.clone(),
            data:Vec::new()
        }
    }

    pub fn write_to_file(&self, path: String) -> bool {
        let mut writer = match csv::WriterBuilder::new()
            .delimiter(b',')
            .quote(b'"')
            .has_headers(false) //write manually
            .from_path(path)
        {
            Ok(x) => x,
            Err(_) => {
                return false;
            }
        };

        let mut rows: Vec<&Vec<String>> = Vec::new();
        rows.push(&self.headers);
        for row in &self.data {
            rows.push(row);
        }

        for row in rows {
            let res = writer.write_record(row);

            match res {
                Err(_) => {
                    return false;
                }
                Ok(_) => {}
            }
        }

        true
    }

    fn get_header_column_index(&self, header_label: &str) -> Result<&usize, String> {
        match self.labelmap.get(header_label) {
            None => Err(format!("No header {} found", header_label)),
            Some(x) => Ok(x),
        }
    }

    pub fn get_val(&self, header_label: &String, row: &usize) -> Result<String, String> {
        let index = self.get_header_column_index(header_label)?.to_owned();

        let datarow = match self.data.get(row.to_owned()) {
            None => {
                return Err(format!("No data row {} exists", row));
            }
            Some(x) => x.to_owned(),
        };

        let cell = datarow.get(index);

        match cell {
            None => Err(format!("No val for {} in row {} ", header_label, row)),
            Some(val) => Ok(val.to_owned()),
        }
    }

    pub fn set_val(
        &mut self,
        header_label: &String,
        row: &usize,
        val: &String,
    ) -> Result<(), String> {
        let index = self.get_header_column_index(header_label)?.to_owned();

        let datarow = match self.data.get_mut(row.to_owned()) {
            None => {
                return Err(format!("No data row {} exists", row));
            }
            Some(x) => x,
        };

        datarow[index] = val.to_owned();
        Ok(())
    }

    pub fn row_indices(&self) -> Range<usize> {
        0..self.data.len()
    }

    pub fn get_keyed_column_sample_map(
        &self,
        key_header_label: &String,
    ) -> Result<HashMap<String, usize>, String> {
        let mut retval: HashMap<String, usize> = HashMap::new();
        for row_i in self.row_indices() {
            let key_value = self.get_val(key_header_label, &row_i);
            match key_value {
                Err(x) => return Err(x),
                Ok(key_value) => {
                    if key_value.is_empty() {
                        eprintln!("Row {} header {} is empty.", row_i, key_header_label);
                    } else {
                        retval.insert(key_value, row_i);
                    }
                }
            }
        }

        Ok(retval)
    }

    pub fn for_each<F>(
        &self,
        headers:Vec<String>,
        mut func: F
    ) -> Result<(),String>
    where F:FnMut(&Vec<String>)->()
    {
        let mut header_indices:Vec<&usize>=Vec::with_capacity(headers.len());
        let mut condensed:Vec<String>=Vec::with_capacity(headers.len());

        for header in headers
        {
            header_indices.push(self.get_header_column_index(header.as_str())?);
            condensed.push("".to_string());
        }
        
        for row in &self.data
        {
            for n in 0..header_indices.len()
            {
                let val = row.get(n).expect("Invalid row index").to_string();
                condensed[n]=val;
            }
            func(&condensed);
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn pushrow(&mut self, row: Vec<String>) {
        self.data.push(row);
    }
}
