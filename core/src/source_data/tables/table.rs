use std::collections::HashMap;

pub trait Table<T>
{
    fn build_from_headers_and_row(header_map:&HashMap<String,usize>, row:&Vec<String>)->Result<T,std::io::Error>;

    fn get_from_row_with_header(header:&str, header_map:&HashMap<String,usize>, row:&Vec<String>)->String
    {
        let index=*(header_map.get(header).expect("Should have this header."));
        row.get(index).expect("Should have this member").to_string()
    }

    fn for_each<F>(&self, file_path: &str, func:F) -> Result<(), std::io::Error> 
    where F:Fn(T)->()
    {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .quote(b'"')
            .has_headers(true)
            .from_path(file_path)?;

        let mut headers: Vec<String>=Vec::new();
        let mut labelmap: HashMap<String, usize>=HashMap::new();

        let mut n: usize = 0;
        for header in rdr.headers()?.iter() {
            headers.push(header.to_string());
            labelmap.insert(header.to_string(), n);
            n += 1;
        }

        n=0;
        for record in rdr.records() {
            n+=1;
            let mut row: Vec<String> = Vec::new();
            for cell in record?.iter() {
                row.push(cell.to_string());
            }

            if row.len() != labelmap.keys().len() {
                let mut message: String = "Malformed data. Header length is ".to_string();
                message += &(headers.len().to_string());
                message += " but row ";
                message += &(n.to_string());
                message += " contains ";
                message += &(row.len().to_string());
                message += " items.";
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, message));
            }

            let member:T=Self::build_from_headers_and_row(&labelmap,&row)?;
            func(member);
        }

        Ok(())
    }
}