use std::{error::Error, collections::{hash_map::DefaultHasher, HashMap, HashSet}, hash::Hasher, fs::OpenOptions, io::Write};
use crate::{file_object::FileObject, common::sort_and_convert_to_offset};


pub fn get_columnset_duplicates_count(file_object: &FileObject, column_numbers: &Vec<usize>) -> Result<usize, Box<dyn Error>> {
    let column_offset = sort_and_convert_to_offset(column_numbers);

    let mut testset: HashSet<u64> = HashSet::with_capacity(file_object.total_rows);
    let mut duplicates: usize = 0;
    let rowiterator = file_object.data.lines();
    for row in rowiterator {
        // computes for one set of column numbers on each row
        // could be sped up by working on multiple sets per row so we don't iterate over each row for every set
        let mut s = DefaultHasher::new();
        let mut splitrow = row.split(&file_object.delim);
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

pub fn save_columnset_duplicates(file_object: &FileObject, column_numbers: &Vec<usize>, output_file_location: &str) -> Result<usize, Box<dyn Error>> {
    let column_offset = sort_and_convert_to_offset(column_numbers);
    let mut duplicates_list: Vec<String> = Vec::new();

    let mut testset: HashMap<u64, (bool, String)> = HashMap::with_capacity(file_object.total_rows);
    let mut duplicates: usize = 0;
    let rowiterator = file_object.data.lines();
    for row in rowiterator {
        let mut s = DefaultHasher::new();
        let mut splitrow = row.split(&file_object.delim);
        for x in &column_offset {
            s.write(splitrow.nth(*x).unwrap().as_bytes())
        }
        let val = s.finish();

        let exists_in_testset = testset.contains_key(&val);
        let duplicate_data: Option<(bool, String)> = testset.insert(val, (exists_in_testset, row.to_string()));
        if duplicate_data != None {
            if duplicate_data.as_ref().unwrap().0 == false {
                duplicates_list.push(row.clone().to_owned().replace(&file_object.internal_delimiter, &file_object.delim)+"\n");
            }
            duplicates_list.push(duplicate_data.unwrap().1.replace(&file_object.internal_delimiter, &file_object.delim)+"\n");
            duplicates += 1;
        }
    }
    duplicates_list.sort_unstable();

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(output_file_location)?;
    
    output_file.write((file_object.headers.clone().join(&file_object.delim)+"\n").as_bytes())?;
    output_file.write(duplicates_list.join("").as_bytes())?;

    Ok(duplicates)
}