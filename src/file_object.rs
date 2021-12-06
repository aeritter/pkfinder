use std::{fs, collections::{hash_map::DefaultHasher, HashSet}, error::Error, hash::{Hasher, Hash}};

pub struct FileObject {
    pub data: String,
    pub newline: String,
    pub delim: String,
    pub total_columns: usize,
    pub headers: Vec<String>,
    pub total_rows: usize,
    // short_circuit: bool,
}


impl FileObject {
    pub fn new(file_path: &str) -> FileObject {
        let file_contents = fs::read_to_string(file_path)
            .expect("Problem loading file");
        let newline_types = vec!["\r\n", "\r", "\n"];
        let delimiter_types = vec![",", "\t"];
    
        let testdata = &file_contents[0..1000];
        let mut newline_type: String = String::new();
        for newline in newline_types {
            if testdata.contains(newline) {
                newline_type = newline.to_string();
                break;
            }
        }
        

        // Find Delimiter
        // NOTE: need to account for escaped characters & characters between quotes
        let rows = file_contents.lines();
        let test: Vec<&str> = rows.clone().take(5).collect();
        let mut delimiter_type: String = String::new();
        for delim in delimiter_types {
            if test[0].contains(delim) {
                let delim_test_count: Vec<usize> = test.iter()
                    .map(|x| x.matches(delim).count())
                    .collect();
                if delim_test_count.iter().all(|x| x == &delim_test_count[0]) {
                    delimiter_type = delim.to_string();
                    break;
                }
            }
        }
        
        // Find Headers
        let headers: Vec<String> = rows.clone().next().unwrap().split(&delimiter_type).map(|x| x.to_string()).collect();
        let num_of_rows = rows.count();
        
    

        FileObject {
            data: file_contents,
            newline: newline_type,
            delim: delimiter_type,
            total_columns: headers.len(),
            headers: headers,
            total_rows: num_of_rows,
            // short_circuit: true,
        }
    }


    pub fn get_duplicate_row_count(&self) -> Result<usize, Box<dyn Error>> {
        // Determines how many duplicate rows exist. Uses the entire contents of each row.
        fn generate_hash_from_str(hashstr: &str) -> u64 {
            let mut s = DefaultHasher::new();
            hashstr.hash(&mut s);
            s.finish()
        }
        let rows = self.data.lines();
        let num_of_rows = rows.clone().count();
        let mut datahashset = HashSet::with_capacity(num_of_rows);
        rows.into_iter()
            .all(|x| datahashset.insert(generate_hash_from_str(x)));
        let dupecount = num_of_rows - datahashset.len();
        // creating a vec to sort() and dedup() is 4x slower than using a hashset.
        println!("Total true duplicates: {}", dupecount);
        Ok(dupecount)
    }

    
    pub fn header_index_to_string(&self, column_numbers: Vec<usize>) -> Vec<String> {
        column_numbers.iter().map(|x| self.headers[*x].clone()).collect()
    }


    pub fn print_details(&self) {
        // println!("Newline format: {:?}", self.newline);
        println!("Delimiter: {:?}", self.delim);
        println!("Column count: {:?}", self.total_columns);
        println!("Row count: {:?}", self.total_rows);
    }


    pub fn get_total_columns(&self) -> usize {
        return self.total_columns.clone();
    }

}
