use std::{error::Error};
use rayon::{iter::{ParallelIterator, IntoParallelRefIterator}};

use crate::file_object::FileObject;

use super::get_columnset_duplicates_count;

pub fn get_known_keys(file_object: &FileObject, dupe_count: usize) -> Result<Vec<usize>, Box<dyn Error>> {
    // Find keys from the top down -- if removing a single column leads to an increase in duplicates
    // then we know that the column is necessary and should be considered part of the primary key set.
    // POTENTIAL ERROR: the only case in which this may not work is if two keys increase duplicate rows but the duplicates of one key are a subset of duplicates from the other.
    // That case should be extremely rare, but the code should be still be adjusted to account for that.
    
    let answer_dataset: Vec<(usize, usize)> = (0..file_object.total_columns).collect::<Vec<usize>>()
        .par_iter()
        .map(|colnum| {
            let test_set = (0..file_object.total_columns).filter(|x| x != colnum).collect();
            let answer = get_columnset_duplicates_count(file_object, &test_set).unwrap();
            let current_col = colnum.clone();
            return (current_col, answer);
        })
        .collect();

    let known_keys = answer_dataset
        .iter()
        .filter(|col_data| col_data.1 > dupe_count)
        .map(|col_num| col_num.0)
        .collect();
        
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
