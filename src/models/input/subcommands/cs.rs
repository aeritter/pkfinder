use clap::Parser;
use itertools::any;
use pyo3::FromPyObject;

use crate::{file_object::FileObject, calculators::{get_columnset_duplicates_count, save_columnset_duplicates}};

#[derive(Debug, Parser, FromPyObject)]
pub struct ColumnSetOptions {
    /// Find duplicates for the given set of column numbers separated by spaces.
    /// Example: -s 1 3 4 8
    pub specific_columns: Vec<usize>,
    /// Output list of duplicates found to the specified file.
    #[clap(short, long = "output_path")]
    pub output: Option<String>,
}

impl ColumnSetOptions {
    pub fn run(&self, data: &FileObject) {
        
        if any(&self.specific_columns, |x| x < &1) {
            println!("All specified columns must be between 1 and {}.", data.total_columns);
            return ();
        }

        // convert to 0-based indexing
        let test_set: Vec<usize> = self.specific_columns.iter().map(|x| x - 1).collect();
        println!("\nTesting these columns: {:?}", data.header_index_to_string(&test_set));

        // Write duplicates to file if output option given
        match &self.output {
            Some(output_location) => println!("\nWrote duplicates to file: {:?}\nDuplicates found: {:?}", &output_location, save_columnset_duplicates(&data, &test_set, &output_location).unwrap()),
            None => println!("\nDuplicates found: {:?}", get_columnset_duplicates_count(&data, &test_set).unwrap())
        }     
    }
}
