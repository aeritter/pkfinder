use std::{error::Error, collections::{hash_map::DefaultHasher, HashSet}, hash::Hasher};
use crate::{file_object::FileObject, common::sort_and_convert_to_offset};


pub fn get_columnset_duplicates(file_object: &FileObject, column_numbers: &Vec<usize>) -> Result<usize, Box<dyn Error>> {
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