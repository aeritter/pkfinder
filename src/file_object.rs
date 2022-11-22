use std::{ collections::{hash_map::DefaultHasher, HashSet}, error::Error, io::{BufReader, BufRead, Seek}, fs::File, hash::{Hash, Hasher}, os::windows::prelude::FileExt};
use itertools::Itertools;
use rayon::prelude::*;

use csv::ReaderBuilder;


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
    pub fn from_file(file_path: &str, delimiter: &str, deduplicate: bool, skip_lines: usize) -> Result<FileObject, Box<dyn Error>> {
        let original_delimiter = delimiter.to_string();

        let internal_delimiter: String;
        let (headers, mut file_contents) = match FileObject::file_to_vec_simplified(file_path, &original_delimiter, skip_lines) {
            Ok(data) => {
                internal_delimiter = original_delimiter.clone();
                data
            },
            Err(_) => {
                let new_delimiter = "\0".to_string();
                internal_delimiter = new_delimiter;
                match FileObject::file_to_vec(file_path, &original_delimiter, &"\0".to_string(), skip_lines) {
                    Ok(data) => data,
                    Err(_) => panic!("Could not generate list of rows")
                }
            }
        };

        // let headers: Vec<String> = FileObject::get_headers_from_row(&file_contents[0], &internal_delimiter)?;

        let duplicates: usize;
        if deduplicate {
            let pre_dedup_size: usize = file_contents.len();
            file_contents.par_sort_unstable();
            file_contents.dedup();
            duplicates = pre_dedup_size - file_contents.len();
        } else {
            duplicates = 0;
        }

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


    fn file_to_vec_simplified(file_path: &str, current_delimiter: &String, skip_lines: usize) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
        let file_reader = File::open(file_path)?;
        let mut reader = BufReader::new(file_reader)
            .lines();
        reader.advance_by(skip_lines);
        let headers: Vec<String> = reader.next().unwrap().unwrap().split(current_delimiter).map(|x| x.to_string()).collect();
        let delim_match_num: usize = headers.len();
        let result: Result<Vec<String>, Box<dyn Error>> = reader
            .map(|x| {
                let c = x.unwrap();
                if c.matches(current_delimiter).count() != delim_match_num {
                    return Ok(c)
                } else {
                    return Err("Verification failed on simplified file-to-vec conversion".into())
                }
            })
            .collect();
        Ok((headers, result?))
    }

    fn file_to_vec(file_path: &str, current_delimiter: &String, new_delimiter: &String, skip_lines: usize) -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {

        //skip lines
        let reader = File::open(file_path).unwrap();
        let mut buf_reader = BufReader::new(reader);
        let mut buf = String::new();
        let mut start_position: u64 = 0;
        let mut lines_to_skip = skip_lines;
        while let Ok(num) = buf_reader.read_line(&mut buf) {
            if lines_to_skip == 0 {
                break;
            }
            start_position += num as u64;
            lines_to_skip -= 1;
        }

        //begin compiling vec
        let mut reader = File::open(file_path).unwrap();
        reader.seek(std::io::SeekFrom::Start(start_position)).unwrap();

        let mut csv_reader = ReaderBuilder::new()
            .delimiter(current_delimiter.as_bytes()[0])
            .escape(Some(b'\\'))
            .has_headers(true)
            .from_reader(reader);
        let headers: Vec<String> = csv_reader.headers()?.iter().map(|x| x.to_string()).collect();
        let result = csv_reader
            .records()
            .map(|record| record
                .unwrap()
                .iter()
                .join(&new_delimiter)
            )
            .collect();
        Ok((headers, result))   
    }


    // pub fn get_duplicate_row_count(rows: &Vec<String>) -> Result<usize, Box<dyn Error>> {
    //     // Determines how many duplicate rows exist. Uses the entire contents of each row.
    //     fn generate_hash_from_str(hashstr: &str) -> u64 {
    //         let mut s = DefaultHasher::new();
    //         hashstr.hash(&mut s);
    //         s.finish()
    //     }
    //     let num_of_rows = rows.len();
    //     let mut datahashset = HashSet::with_capacity(num_of_rows);
    //     for row in rows {
    //         datahashset.insert(generate_hash_from_str(&row));
    //     }
    //     let dupecount = num_of_rows - datahashset.len();
    //     // creating a vec to sort() and dedup() is 4x slower than using a hashset.
    //     // println!("Total true duplicates: {}", dupecount);
    //     Ok(dupecount)
    // }

    
    pub fn header_index_to_string(&self, column_numbers: &Vec<usize>) -> Vec<String> {
        column_numbers.iter().map(|x| self.headers[*x].clone()).collect()
    }



}
