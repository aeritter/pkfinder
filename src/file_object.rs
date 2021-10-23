use std::{collections::HashSet, error::Error, fs, vec};
// use crate::pause;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use itertools::Itertools;


pub struct FileObject {
    data: String,
    // newline: String,
    delim: String,
    total_columns: usize,
    headers: Vec<String>,
    total_rows: usize,
    // short_circuit: bool,
    
}

impl FileObject {
    pub fn new(file_path: &str) -> FileObject {
        let file_contents = fs::read_to_string(file_path)
            .expect("Problem loading file");
        // let newline_types = vec!["\r\n", "\r", "\n"];
        let delimiter_types = vec![",", "\t"];
    
        // let test = &file_contents[0..1000];
        // let mut newline_type: String = String::new();
        // for newline in newline_types {
        //     if test.contains(newline) {
        //         newline_type = newline.to_string();
        //         break;
        //     }
        // }
        

        // Find Delimiter

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
            // newline: newline_type,
            delim: delimiter_type,
            total_columns: headers.len(),
            headers: headers,
            total_rows: num_of_rows,
            // short_circuit: true,
        }
    }


    pub fn get_pk(&self, calculate_known_keys: bool) -> Result<Vec<String>, Box<dyn Error>> {
        let dupe_count = self.get_true_duplicates()?;
        let known_keys: Vec<usize>;
        if calculate_known_keys {
            known_keys = self.get_known_keys()?;
        } else {
            known_keys = vec![];
        }

        // Return if the known keys make up the primary key
        if known_keys.len() > 0 && !Self::contains_duplicates(self, &known_keys, Some(dupe_count))? {
            let result = self.header_index_to_string(known_keys);
            return Ok(result)
        }

        // Now we have known keys, but it's possible that more are required to find the full primary key set.
        // We could now search by removing 2 at a time, but that would still be a lot of data to process
        // so we now work our way up by starting with the known keys, and add combinations of increasing size
        // until we find all of our keys.
        let columns_to_check: Vec<usize> = (0..self.total_columns).filter(|x| !known_keys.contains(x)).collect();
        for combination_size in 1..self.total_columns-known_keys.len() {
            for combination in columns_to_check.clone().into_iter().combinations(combination_size) {
                // if combination is superset of known match, then {
                //     continue
                // }
                let mut new_combination: Vec<usize> = combination.clone();
                new_combination.extend_from_slice(&known_keys);
                new_combination.sort_unstable();
                if !Self::contains_duplicates(self, &new_combination, Some(dupe_count))? {
                    let result = self.header_index_to_string(new_combination);
                    return Ok(result);
                }
            }
        }
        return Err("PK not found")?;
    }


    fn get_true_duplicates(&self) -> Result<usize, Box<dyn Error>> {
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


    fn get_known_keys(&self) -> Result<Vec<usize>, Box<dyn Error>> {
        // Find keys from the top down -- if removing a single column leads to an increase in duplicates
        // then we know that the column is necessary and should be considered part of the primary key set.
        // POTENTIAL ERROR: the only case in which this may not work is if two keys increase duplicate rows but the duplicates of one key are a subset of duplicates from the other.
        // That case should be extremely rare, but the code should be still be adjusted to account for that.
        let mut test_set: Vec<Vec<usize>> = Vec::new();
        for index in 0..self.total_columns {
            test_set.extend(vec![(0..self.total_columns).filter(|column_num| column_num != &index).collect()]);
        }
        
        let mut known_keys: Vec<usize> = vec![];
        let mut testset: Vec<HashSet<u64>> = vec![HashSet::with_capacity(self.total_rows); self.total_columns];
        let mut duplicates: Vec<usize> = vec![0; self.total_columns];
        let rowiterator = self.data.lines();
        for row in rowiterator {
            let splitrow = row.split(&self.delim);
            let mut hasher_set: Vec<DefaultHasher> = vec![DefaultHasher::new(); self.total_columns];
            // add contents of all fields to the hasher, except for fields where the column number matches the set number
            for (colnum, contents) in splitrow.enumerate() {
                for (i, x) in hasher_set.iter_mut().enumerate() {
                    if i != colnum {
                        x.write(contents.as_bytes());
                    }
                }
            }
            // Finish generating hash, then add to hashset. If it can't be added because it's a duplicate, then we
            // have a match for one of our known keys.
            for (i, x) in testset.iter_mut().enumerate() {
                let newval = x.insert(hasher_set[i].finish());
                if newval != true {
                    duplicates[i] += 1;
                }
            }
        }
        for (i, x) in duplicates.iter().enumerate() {
            if x > &0 {
                known_keys.push(i.clone())
            }
        }
        Ok(known_keys)
    }


    // fn get_known_keys_old(&self, dupe_count: Option<usize>) -> Result<Vec<usize>, Box<dyn Error>> {
    //     // Find known keys one at a time -- iterates over the entire file for every column set. Slower than iterating
    //     // over the file once and calculating values for every column set at the same time.
    //     let mut known_keys = vec![];
    //     for index in 0..self.total_columns {
    //         let test_set: Vec<usize> = (0..self.total_columns).filter(|column_num| column_num != &index).collect();
    //         if self.contains_duplicates(&test_set, dupe_count)? {
    //             println!("Found result, column number: {:?}",index);
    //             known_keys.push(index)
    //         }
    //     }
    //     Ok(known_keys)
    // }


    pub fn contains_duplicates(&self, column_numbers: &Vec<usize>, dupe_count: Option<usize>) -> Result<bool, Box<dyn Error>> {
        let column_offset = self.sort_and_convert_to_offset(column_numbers);
        let mut testset: HashSet<u64> = HashSet::with_capacity(self.total_rows);
        let mut duplicates: usize = 0;
        let mut answer: bool = false;
        let rowiterator = self.data.lines();
        for row in rowiterator {
            // computes for one set of column numbers on each row
            // could be sped up by working on multiple sets per row so we don't iterate over each row for every set
            let mut s = DefaultHasher::new();
            let mut splitrow = row.split(&self.delim);
            for x in &column_offset {
                s.write(splitrow.nth(*x).unwrap().as_bytes())
            }
            let val = s.finish();
            if testset.insert(val) != true {
                duplicates += 1;
                if duplicates > dupe_count.unwrap_or(0) { //short-circuit to avoid unnecessary checks
                    answer = true;
                    break;
                }
            }
        }
        Ok(answer.clone())
    }


    pub fn get_columnset_duplicates(&self, column_numbers: &Vec<usize>) -> Result<usize, Box<dyn Error>> {
        let column_offset = self.sort_and_convert_to_offset(column_numbers);

        let mut testset: HashSet<u64> = HashSet::with_capacity(self.total_rows);
        let mut duplicates: usize = 0;
        let rowiterator = self.data.lines();
        for row in rowiterator {
            // computes for one set of column numbers on each row
            // could be sped up by working on multiple sets per row so we don't iterate over each row for every set
            let mut s = DefaultHasher::new();
            let mut splitrow = row.split(&self.delim);
            for x in &column_offset {
                s.write(splitrow.nth(*x).unwrap().as_bytes())
            }
            let val = s.finish();
            if !testset.insert(val) {
                duplicates += 1;
            }
        }
        Ok(duplicates)
    }


    fn sort_and_convert_to_offset(&self, column_numbers: &Vec<usize>) -> Vec<usize> {

        let mut column_numbers_sorted: Vec<usize> = column_numbers.clone();
        column_numbers_sorted.sort_unstable();
        let mut column_numbers_offset: Vec<usize> = vec![];
        let mut previous_number: usize = 0;
        for x in column_numbers_sorted {
            column_numbers_offset.push(x-&previous_number);
            previous_number = x+1;
        }
        return column_numbers_offset;

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


    pub fn header_index_to_string(&self, column_numbers: Vec<usize>) -> Vec<String> {
        column_numbers.iter().map(|x| self.headers[*x].clone()).collect()
    }

}
