use clap::Parser;
use pyo3::FromPyObject;

use crate::{
    calculators::{get_duplicates_count, get_known_keys, get_pk},
    file_object::FileObject,
};

#[derive(Debug, Parser, FromPyObject)]
pub struct PKOptions {
    /// Calculate known keys first. Faster if the result has many primary keys. (looks for which columns cannot be omitted without increasing duplicate count)
    #[clap(short)]
    pub calculate_known_keys: bool,
    /// Calculate duplicate count for every combination up to max_combination_size and write to the specified file.
    #[clap(short, long = "output_path")]
    pub output: Option<String>,
    /// Set max combination size for duplicate count (-o option)
    #[clap(short, long, default_value = "5")]
    pub max_combination_size: usize,
    /// Print column numbers of found primary keys in output
    #[clap(short = 'n', long)]
    pub print_column_numbers: bool,
}

impl PKOptions {
    pub fn run(&self, data: &FileObject) {
        if self.calculate_known_keys {
            // let duplicate_row_count = data.get_duplicate_row_count().unwrap_or(0);
            let known_keys = get_known_keys(&data, 0).expect("Could not calculate known keys.");
            println!(
                "\nKnown keys: {:?}",
                data.header_index_to_string(&known_keys)
            );
            let primary_keys = get_pk(&data, 0, known_keys).expect("Couldn't find primary key combination.");
            if self.print_column_numbers {
                println!("\nPrimary Key Combination: {}", self.map_keys_to_column_numbers(&data, primary_keys))
            } else {
                println!("\nPrimary Key Combination: {:?}", primary_keys)
            }
        };
        match &self.output {
            Some(output_location) => {
                get_duplicates_count(&data, &output_location, self.max_combination_size)
                    .expect("Encountered a problem while running the output combination option.")
            }
            None => {
                if !self.calculate_known_keys {
                    // let duplicate_row_count = data.get_duplicate_row_count().unwrap_or(0);
                    let primary_keys = get_pk(&data, 0, vec![]).expect("Couldn't find primary key combination.");
                    if self.print_column_numbers {
                        println!("\nPrimary Key Combination: {}", self.map_keys_to_column_numbers(&data, primary_keys))
                    } else {
                        println!("\nPrimary Key Combination: {:?}", primary_keys)
                    }
                }
            }
        }
    }

    fn map_keys_to_column_numbers(&self, data: &FileObject, keys: Vec<String>) -> String {
        keys.iter()
        .map(|key| 
            format!(
                "\n{}: {}",
                (data.string_to_header_index(&key).expect(format!("Expected {key} to exist in headers list").as_str()) + 1)
                .to_string(),
                key
            )
        )
        .collect::<String>()
    }
}
