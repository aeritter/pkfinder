use std::{error::Error, collections::{HashSet, hash_map::DefaultHasher}, hash::Hasher};
use itertools::Itertools;
use crate::{file_object::FileObject, common::{sort_and_convert_to_offset}};
use rayon::prelude::*;


pub fn get_pk(file_object: &FileObject, dupe_count: usize, known_keys: Vec<usize>) -> Result<Vec<String>, Box<dyn Error>> {
    // Return if the known keys make up the primary key
    if known_keys.len() > 0 && !contains_duplicates(file_object, &known_keys, dupe_count)? {
        let result = file_object.header_index_to_string(&known_keys);
        return Ok(result)
    }

    // Now we have known keys, but it's possible that more are required to find the full primary key set.
    // We could now search by removing 2 at a time, but that would still be a lot of data to process
    // so we now work our way up by starting with the known keys, and add combinations of increasing size
    // until we find all of our keys.
    let columns_to_check: Vec<usize> = (0..file_object.total_columns).filter(|x| !known_keys.contains(x)).collect();
    for combination_size in 1..file_object.total_columns-known_keys.len() {
        let answer = columns_to_check.clone()
            .into_iter()
            .combinations(combination_size)
            .par_bridge()
            .find_map_any(|combination| 
                {
                    let mut new_combination: Vec<usize> = combination.clone();
                    new_combination.extend_from_slice(&known_keys);
                    new_combination.sort_unstable();
                    if !contains_duplicates(file_object, &new_combination, dupe_count).unwrap() {
                        let result = file_object.header_index_to_string(&new_combination);
                        return Some(result);
                    } else {
                        return None;
                    }
                }
            );
        if answer.is_some() {
            return Ok(answer.unwrap());
        }
    }
    return Err("PK not found")?;
}

fn contains_duplicates(file_object: &FileObject, column_numbers: &Vec<usize>, dupe_count: usize) -> Result<bool, Box<dyn Error>> {
    let column_offset = sort_and_convert_to_offset(column_numbers);
    let mut testset: HashSet<u64> = HashSet::with_capacity(file_object.total_rows);
    let mut duplicates: usize = 0;
    let mut answer: bool = false;
    let rowiterator = &file_object.data;
    for row in rowiterator {
        // computes for one set of column numbers on each row
        // could be sped up by working on multiple sets per row so we don't iterate over each row for every set
        let mut s = DefaultHasher::new();
        let mut splitrow = row.split(&file_object.internal_delimiter);
        for x in &column_offset {
            s.write(splitrow.nth(*x).unwrap().as_bytes())
        }
        let val = s.finish();
        if testset.insert(val) != true {
            duplicates += 1;
            if duplicates > dupe_count { //short-circuit to avoid unnecessary checks
                answer = true;
                break;
            }
        }
    }
    Ok(answer.clone())
}

