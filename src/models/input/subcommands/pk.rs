use clap::Parser;
use pyo3::FromPyObject;

use crate::{calculators::{get_known_keys, get_pk, get_duplicates_count}, file_object::FileObject};

#[derive(Debug, Parser, FromPyObject)]
pub struct PKOptions {
    /// Calculate known keys first. Faster if the result has many primary keys.
    #[clap(short)]
    pub calculate_known_keys: bool,
    /// Calculate duplicate count for every combination up to max_combination_size and write to the specified file.
    #[clap(short, long = "output_path")]
    pub output: Option<String>,
    /// Set max combination size for duplicate count (-o option)
    #[clap(short, long, default_value = "5")]
    pub max_combination_size: usize,
}

impl PKOptions {
    pub fn run(&self, data: &FileObject) {
        if self.calculate_known_keys {
            // let duplicate_row_count = data.get_duplicate_row_count().unwrap_or(0);
            let known_keys = get_known_keys(&data, 0).expect("Could not calculate known keys.");
            println!("\nKnown keys: {:?}", data.header_index_to_string(&known_keys));
            println!("\nPrimary Key Combination: {:?}", get_pk(&data, 0, known_keys).expect("Couldn't find primary key combination."))
        };
        match &self.output {
            Some(output_location) => get_duplicates_count(&data, &output_location, self.max_combination_size).expect("Encountered a problem while running the output combination option."),
            None => {
                if ! self.calculate_known_keys {
                    // let duplicate_row_count = data.get_duplicate_row_count().unwrap_or(0);
                    println!("\nPrimary Key Combination: {:?}", get_pk(&data, 0, vec![]).expect("Couldn't find primary key combination."));
                }
            }
        }
    }
}
