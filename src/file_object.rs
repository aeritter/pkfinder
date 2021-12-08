use std::{ collections::{hash_map::DefaultHasher, HashSet, HashMap}, error::Error, hash::{Hasher, Hash}, io::{BufReader, BufRead}, fs::{self, File}};
use itertools::Itertools;
use regex::{Regex};
use csv::{ReaderBuilder};

pub struct FileObject {
    pub data: String,
    pub newline: String,
    pub delim: String,
    pub internal_delimiter: String,
    pub total_columns: usize,
    pub headers: Vec<String>,
    pub total_rows: usize,
}


impl FileObject {
    pub fn new(file_path: &str) -> FileObject {
        
        let file_contents: String;

        // Find Delimiter
        // NOTE: need to account for escaped characters & characters between quotes
        let mut delimiter_type: String = String::new();
        let internal_delimiter: String;

        let delimiter_types = vec![",", "\t"];
        let file_reader = File::open(file_path).unwrap();
        let reader = BufReader::new(file_reader);
        let delim_finding_data: Vec<String> = reader.lines().take(5).map(|x| x.unwrap()).collect();
        for delim in delimiter_types {
            if delim_finding_data[0].contains(delim) {
                let delim_test_count: Vec<usize> = delim_finding_data.iter()
                    .map(|x| x.matches(delim).count())
                    .collect();
                if delim_test_count.iter().all(|x| x == &delim_test_count[0]) {
                    delimiter_type = delim.to_string();
                    break;
                }
            }
        }
        if delimiter_type.is_empty() {
            // regex: Non-word, non-space, non-period value is grabbed, where it's followed by a quote or word character, followed by anything, then mirrored (palindrome)
            // if the value grabbed at the front is equivalent to the value grabbed at the back, add that value as a potential delimiter, then attempt
            // to use the value that appeared the most as the delimiter.
            let regex_string = Regex::new(r#"(?P<delim1>[^\w \.\\"])(?:"|\w).*?(?:"|\w)(?P<delim2>[^\w \.\\"])"#).unwrap();
            let mut delim_hashmap = HashMap::new();
            for row in &delim_finding_data {
                for result in regex_string.captures_iter(&row) {
                    if &result["delim1"] == &result["delim2"] {
                        let delim1 = result["delim1"].to_string();
                        let y = delim_hashmap.entry(delim1).or_insert(1);
                        *y += 1;
                    }
                }
            }
            let delim_guess = delim_hashmap
                .iter()
                .max_by(|x, y| x.1.cmp(&y.1))
                .map(|(k, _v)| k)
                .unwrap();
            delimiter_type = delim_guess.clone();

            // TODO: test the delim here and if it's wrong, test the one with the next-highest occurences or attempt to guess on a larger data set

            let mut csv_reader = ReaderBuilder::new()
                .delimiter(delimiter_type.as_bytes()[0])
                .escape(Some(b'\\'))
                .has_headers(false)
                .from_reader(File::open(file_path).unwrap());
                
            file_contents = csv_reader
                .records()
                .map(|record| record
                    .unwrap()
                    .iter()
                    .join("\0")
                )
                .join("\n");
                
            internal_delimiter = "\0".to_string()
        } else {
            internal_delimiter = delimiter_type.clone();
            file_contents = fs::read_to_string(file_path)
                .expect("Problem loading file");
        }

        let num_of_rows = file_contents.lines().count();

        let newline_types = vec!["\r\n", "\r", "\n"];
        let testdata = &file_contents[0..1000];
        let mut newline_type: String = String::new();
        for newline in newline_types {
            if testdata.contains(newline) {
                newline_type = newline.to_string();
                break;
            }
        }

        if delimiter_type.is_empty() {
            panic!("Could not determine delimiter.")
        }

        // Find Headers
        let headers: Vec<String> = file_contents.lines().next().unwrap().split(&internal_delimiter).map(|x| x.to_string()).collect();
        let total_columns = headers.len();

            

        FileObject {
            data: file_contents,
            newline: newline_type,
            delim: delimiter_type,
            internal_delimiter: internal_delimiter,
            total_columns: total_columns,
            headers: headers,
            total_rows: num_of_rows,
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

    
    pub fn header_index_to_string(&self, column_numbers: &Vec<usize>) -> Vec<String> {
        column_numbers.iter().map(|x| self.headers[*x].clone()).collect()
    }


    pub fn print_details(&self) {
        println!("Newline format: {:?}", self.newline);
        println!("Delimiter: {:?}", self.delim);
        println!("Column count: {:?}", self.total_columns);
        println!("Row count: {:?}", self.total_rows);
    }

    pub fn print_column_numbers(&self) {
        println!();
        for (index, column) in self.headers.iter().enumerate() {
            println!("{}: {}", index+1, column)
        }
        println!();
    }

}
