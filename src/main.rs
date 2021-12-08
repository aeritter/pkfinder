mod file_object;
mod common;
mod calculators;
use std::path::Path;
use std::time::SystemTime;
use itertools::any;
use structopt::StructOpt;
use crate::calculators::{get_columnset_duplicates, get_pk, get_known_keys};
use crate::file_object::FileObject;



#[derive(Debug, StructOpt)]
#[structopt(name = "Primary Key finder", about = "Assists with finding the best primary key combination for a given dataset.\n
WARNING: The given Primary Key combination is simply the smallest combination of columns that result in as few duplicates as possible. \
You will have to look at the data itself and understand what the data means to determine if the result given needs to be adjusted.
", author = "by Andrew Ritter")]
struct Config {
    /// Location of the file to process
    file: String,

    /// Find duplicates for the given set of column numbers separated by spaces.
    #[structopt(short, long)]
    specific_columns: Vec<usize>,

    /// Calculate known keys first. Faster if the result has many primary keys.
    #[structopt(short)]
    calculate_known_keys: bool,

    /// Print file details (delimiter, column & row counts)
    #[structopt(short, long)]
    properties: bool,

    /// Print column numbers and the associated headers
    #[structopt(short = "n", long)]
    column_numbers: bool,

    /// Debugging (only displays total run time at the moment)
    #[structopt(short, long)]
    debug: bool
}



fn main() {
    let ctime = SystemTime::now();

    let options = Config::from_args();
    println!("Using file: {}\n", Path::new(& options.file).file_name().unwrap().to_str().unwrap());
    let data = FileObject::new(&options.file);
    
    if options.properties {
        data.print_details();
    }

    if options.column_numbers {
        data.print_column_numbers()
    }

    if !options.specific_columns.is_empty() {
        if any(&options.specific_columns, |x| x < &1) {
            println!("All specified columns must be between 1 and {}.", data.total_columns);
        } else {
            // convert to 0-based indexing
            let test_set: Vec<usize> = options.specific_columns.iter().map(|x| x - 1).collect();
            println!("Testing these columns: {:?}", data.header_index_to_string(&test_set));
            println!("Duplicates found: {:?}", get_columnset_duplicates(&data, &test_set).unwrap())
        }
    } else if options.calculate_known_keys {
        let duplicate_row_count = data.get_duplicate_row_count().unwrap_or(0);
        let known_keys = get_known_keys(&data, duplicate_row_count).expect("Could not calculate known keys.");
        println!("\nKnown keys: {:?}", data.header_index_to_string(&known_keys));
        println!("\nPrimary Key Combination: {:?}", get_pk(&data, duplicate_row_count, known_keys).expect("Couldn't find primary key combination."))
    } else if !options.file.is_empty() {
        let duplicate_row_count = data.get_duplicate_row_count().unwrap_or(0);
        println!("\nPrimary Key Combination: {:?}", get_pk(&data, duplicate_row_count, vec![]).expect("Couldn't find primary key combination."))
    }
    println!();

    if options.debug {
        println!("Total run time: {:?}\n", SystemTime::now().duration_since(ctime).unwrap())
    }
}
