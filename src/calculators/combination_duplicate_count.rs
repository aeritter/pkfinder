use std::{error::Error, collections::{hash_map::DefaultHasher, HashMap, HashSet}, hash::Hasher, fs::OpenOptions, io::Write};
use itertools::Itertools;
use rayon::slice::ParallelSliceMut;

use crate::{file_object::FileObject, common::sort_and_convert_to_offset};


pub fn get_columnset_duplicates_count(file_object: &FileObject, column_numbers: &Vec<usize>) -> Result<usize, Box<dyn Error>> {
    let column_offset = sort_and_convert_to_offset(column_numbers);

    let mut testset: HashSet<u64> = HashSet::with_capacity(file_object.total_rows);
    let mut duplicates: usize = 0;
    let rowiterator = &file_object.data;
    let delimiter = &file_object.internal_delimiter.as_bytes()[0];
    for row in rowiterator {
        // computes for one set of column numbers on each row
        // could be sped up by working on multiple sets per row so we don't iterate over each row for every set
        let mut s = DefaultHasher::new();
        let mut splitrow = row.as_bytes().split(|f| f == delimiter);
        for x in &column_offset {
            s.write(splitrow.nth(*x).unwrap())
        }
        let val = s.finish();
        if !testset.insert(val) {
            duplicates += 1;
        }
    }
    Ok(duplicates)
}

pub fn save_columnset_duplicates(file_object: &FileObject, column_numbers: &Vec<usize>, output_file_location: &str) -> Result<usize, Box<dyn Error>> {
    let column_offset = sort_and_convert_to_offset(column_numbers);
    let mut duplicates_list: Vec<(String, String)> = Vec::new();

    let mut testset: HashMap<u64, (bool, String, String)> = HashMap::with_capacity(file_object.total_rows);
    let mut duplicates: usize = 0;
    let rowiterator = &file_object.data;
    let delimiter = &file_object.internal_delimiter.as_bytes()[0];
    for row in rowiterator {
        let mut s = DefaultHasher::new();
        let mut splitrow = row.as_bytes().split(|f| f == delimiter);
        let mut keycombo: Vec<u8> = Vec::new();
        for x in &column_offset {
            let data = splitrow.nth(*x).unwrap();
            keycombo.extend(data);
            s.write(data)
        }
        let val = s.finish();
        let keycombo: String = String::from_utf8(keycombo)?;

        let exists_in_testset = testset.contains_key(&val);
        let duplicate_data: Option<(bool, String, String)> = testset.insert(val, (exists_in_testset, row.to_string(), keycombo.to_string()));
        if duplicate_data != None {
            if duplicate_data.as_ref().unwrap().0 == false {
                duplicates_list.push(
                    (row
                        .clone()
                        .to_owned()
                        .split(&file_object.internal_delimiter)
                        .map(|x| {
                            if x.contains(&file_object.display_delimiter) {
                                return "\"".to_string()+x+"\""
                            } else {
                                return x.to_string()
                            }
                        })
                        .join(&file_object.display_delimiter)
                        + "\n"
                    , keycombo.to_string()
                    )
                );
            }
            duplicates_list.push(
                (duplicate_data
                    .as_ref()
                    .unwrap()
                    .1
                    .split(&file_object.internal_delimiter)
                    .map(|x| {
                        if x.contains(&file_object.display_delimiter) {
                            return "\"".to_string()+x+"\""
                        } else {
                            return x.to_string()
                        }
                    })
                    .join(&file_object.display_delimiter)
                    + "\n"
                , duplicate_data.unwrap().2));
            duplicates += 1;
        }
    }
    duplicates_list.par_sort_unstable_by_key(|k| k.1.to_owned());

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_file_location)?;
    
    output_file.write((file_object.headers.clone().join(&file_object.display_delimiter)+"\n").as_bytes())?;
    output_file.write(duplicates_list.iter().map(|x| &x.0).join("").as_bytes())?;

    Ok(duplicates)
}