use std::{ collections::{HashMap}, error::Error, io::{BufReader, BufRead}, fs::{File}};
use itertools::Itertools;
use rayon::prelude::*;
use regex::{Regex};
use csv::{ReaderBuilder};

pub struct FileObject {
    pub data: Vec<String>,
    // pub newline: String,
    pub display_delimiter: String,
    pub internal_delimiter: String,
    pub total_columns: usize,
    pub headers: Vec<String>,
    pub total_rows: usize,
}


impl FileObject {
    pub fn from_file(file_path: &str) -> Result<FileObject, Box<dyn Error>> {
        let reader = BufReader::new(File::open(file_path)?);
        let delim_finding_data: Vec<String> = reader.lines().take(5).map(|x| x.expect("Expected UTF-8 file encoding")).collect();

        let original_delimiter = match FileObject::find_delimiter_method_1(&delim_finding_data) {
            Ok(delim) => delim,
            Err(_) => match FileObject::find_delimiter_method_2(&delim_finding_data) {
                Ok(delim) => delim,
                Err(_) => panic!("Could not determine delimiter")
            },
        };

        let internal_delimiter: String;
        let mut file_contents = match FileObject::file_to_vec_simplified(file_path, &original_delimiter) {
            Ok(data) => {
                internal_delimiter = original_delimiter.clone();
                data
            },
            Err(_) => {
                let new_delimiter = "\0".to_string();
                internal_delimiter = new_delimiter;
                match FileObject::file_to_vec(file_path, &original_delimiter, &"\0".to_string()) {
                    Ok(data) => data,
                    Err(_) => panic!("Could not generate list of rows")
                }
            }
        };

        let headers: Vec<String> = FileObject::get_headers_from_row(&file_contents[0], &internal_delimiter)?;

        let pre_dedup_size: usize = file_contents.len();
        file_contents.par_sort_unstable();
        file_contents.dedup();
        let duplicates: usize = pre_dedup_size - file_contents.len();

        let num_of_rows: usize = file_contents.len();
        if duplicates > 0 {
            let duplicates_percentage = ((duplicates as f64 / num_of_rows as f64)*10000 as f64).round() / 10000 as f64;
            println!("~{}% of lines were duplicates ({}/{}), consider adding another column to help differentiate.\n", duplicates_percentage, duplicates, num_of_rows)
        }

        let total_columns = headers.len();

        Ok(
            FileObject {
                data: file_contents,
                // newline: newline_type,
                display_delimiter: original_delimiter,
                internal_delimiter: internal_delimiter,
                total_columns: total_columns,
                headers: headers,
                total_rows: num_of_rows,
            }
        )
    }


    fn get_headers_from_row(row: &String, delimiter: &String) -> Result<Vec<String>, Box<dyn Error>> {
        let result = row
            .split(delimiter)
            .map(|x| x.to_string())
            .collect();
        Ok(result)
    }

    fn find_delimiter_method_1(test_data: &Vec<String>) -> Result<String, String>{
        let delimiter_types = vec![",", "\t"];
        for delim in delimiter_types {
            if test_data[0].contains(delim) {
                let delim_test_count: Vec<usize> = test_data.iter()
                    .map(|x| x.matches(delim).count())
                    .collect();
                if delim_test_count.iter().all(|x| x == &delim_test_count[0]) {
                    return Ok(delim.to_string());
                }
            }
        }
        return Err("Could not find delimiter".to_string())
    }
    
    fn find_delimiter_method_2(test_data: &Vec<String>) -> Result<String, Box<dyn Error>>{
        // regex: Non-word, non-space, non-period value is grabbed, where it's followed by a quote or word character, followed by anything, then mirrored (palindrome)
        // if the value grabbed at the front is equivalent to the value grabbed at the back, add that value as a potential delimiter, then attempt
        // to use the value that appeared the most as the delimiter.
        let regex_string = Regex::new(r#"(?P<delim1>[^\w \.\\"])(?:"|\w).*?(?:"|\w)(?P<delim2>[^\w \.\\"])"#).unwrap();
        let mut delim_hashmap = HashMap::new();
        for row in test_data {
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
        return Ok(delim_guess.clone());
        // TODO: test the delim here and if it's wrong, test the one with the next-highest occurences or attempt to guess on a larger data set
        
    }

    fn file_to_vec_simplified(file_path: &str, current_delimiter: &String) -> Result<Vec<String>, Box<dyn Error>> {
        let file_reader = File::open(file_path)?;
        let result: Vec<String> = BufReader::new(file_reader)
            .lines()
            .map(|x| x.unwrap())
            .collect();
        // // Verification step
        // let delim_match_num: usize = result[0].matches(current_delimiter).count();
        // for line in &result {
        //     if line.matches(current_delimiter).count() != delim_match_num {
        //         return Err("Verification failed on simplified file-to-vec conversion".into())
        //     }
        // };
        Ok(result)
    }

    fn file_to_vec(file_path: &str, current_delimiter: &String, new_delimiter: &String) -> Result<Vec<String>, Box<dyn Error>> {
        let mut csv_reader = ReaderBuilder::new()
            .delimiter(current_delimiter.as_bytes()[0])
            .escape(Some(b'\\'))
            .has_headers(true)
            .from_reader(File::open(file_path).unwrap());
        let result = csv_reader
            .records()
            .map(|record| record
                .unwrap()
                .iter()
                .join(&new_delimiter)
            )
            .collect();
        Ok(result)   
    }



    // pub fn get_duplicate_row_count(&self) -> Result<usize, Box<dyn Error>> {
    //     // Determines how many duplicate rows exist. Uses the entire contents of each row.
    //     fn generate_hash_from_str(hashstr: &str) -> u64 {
    //         let mut s = DefaultHasher::new();
    //         hashstr.hash(&mut s);
    //         s.finish()
    //     }
    //     let rows = &self.data;
    //     let num_of_rows = rows.len();
    //     let mut datahashset = HashSet::with_capacity(num_of_rows);
    //     for row in rows {
    //         datahashset.insert(generate_hash_from_str(&row));
    //     }
    //     let dupecount = num_of_rows - datahashset.len();
    //     // creating a vec to sort() and dedup() is 4x slower than using a hashset.
    //     println!("Total true duplicates: {}", dupecount);
    //     Ok(dupecount)
    // }

    
    pub fn header_index_to_string(&self, column_numbers: &Vec<usize>) -> Vec<String> {
        column_numbers.iter().map(|x| self.headers[*x].clone()).collect()
    }


    pub fn print_details(&self) {
        // println!("Newline format: {:?}", self.newline);
        println!("Delimiter: {:?}", self.display_delimiter);
        println!("Column count: {:?}", self.total_columns);
        println!("Row count: {:?}", self.total_rows);
    }

    pub fn print_column_numbers(&self) {
        println!();
        for (index, column) in self.headers.iter().enumerate() {
            println!("{}: {}", index+1, column)
        }
    }

}
