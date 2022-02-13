use std::{error::Error, collections::{HashSet, hash_map::DefaultHasher}, hash::Hasher, fs::OpenOptions, io::Write};
use itertools::Itertools;
use crate::{file_object::FileObject, common::{sort_and_convert_to_offset}};


pub fn get_duplicates_count(file_object: &FileObject, output_file_location: &String, max_combination_size: usize) -> Result<(), Box<dyn Error>> {
    if max_combination_size > file_object.total_columns {
        let errmsg = format!("Given max combination size is larger than the total number of columns available: {}", file_object.total_columns);
        return Err(errmsg)?;
    }
    let columns_to_check: Vec<usize> = (0..file_object.total_columns).collect();

    let mut output_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(output_file_location)?;
    output_file.write("Combination Size, Headers\n".as_bytes())?;

    for combination_size in 1..max_combination_size+1 {
        println!("Working on combinations of {}", combination_size);
        let mut results: String = String::new();
        for combination in columns_to_check.clone().into_iter().combinations(combination_size) {
            // if combination is superset of known match, then {
            //     continue
            // }
            let combination_dupe_count = count_duplicates(file_object, &combination)?.to_string();
            let result = combination_dupe_count + ", [" + &file_object.header_index_to_string(&combination).join(", ") + "]\n";
            results.push_str(&result)
        }
        output_file.write_all(results.as_bytes())?;
    }
    println!("Done writing to file.");
    return Ok(());
}

fn count_duplicates(file_object: &FileObject, column_numbers: &Vec<usize>) -> Result<usize, Box<dyn Error>> {
    let column_offset = sort_and_convert_to_offset(column_numbers);
    let mut testset: HashSet<u64> = HashSet::with_capacity(file_object.total_rows);
    let mut duplicates: usize = 0;
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
        }
    }
    Ok(duplicates)
}
