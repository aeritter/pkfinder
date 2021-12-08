use std::{collections::{HashSet, hash_map::DefaultHasher}, error::Error, hash::Hasher};
use crate::file_object::FileObject;

pub fn get_known_keys(file_object: &FileObject, dupe_count: usize) -> Result<Vec<usize>, Box<dyn Error>> {
    // Find keys from the top down -- if removing a single column leads to an increase in duplicates
    // then we know that the column is necessary and should be considered part of the primary key set.
    // POTENTIAL ERROR: the only case in which this may not work is if two keys increase duplicate rows but the duplicates of one key are a subset of duplicates from the other.
    // That case should be extremely rare, but the code should be still be adjusted to account for that.
    
    let mut known_keys: Vec<usize> = vec![];
    let mut testset: Vec<HashSet<u64>> = vec![HashSet::with_capacity(file_object.total_rows); file_object.total_columns];
    let mut duplicates: Vec<usize> = vec![0; file_object.total_columns];
    let rowiterator = file_object.data.lines();
    for row in rowiterator {
        let splitrow = row.split(&file_object.internal_delimiter);
        let mut hasher_set: Vec<DefaultHasher> = vec![DefaultHasher::new(); file_object.total_columns];
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
        if x > &dupe_count {
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
