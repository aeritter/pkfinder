mod file_object;
mod common;
mod calculators;
mod models;
use std::path::Path;
use std::sync::Arc;
use std::time::SystemTime;
use itertools::any;
use clap::Parser;
use crate::calculators::{get_columnset_duplicates_count, save_columnset_duplicates, get_pk, get_known_keys, get_duplicates_count};
use crate::models::input::Config;
use crate::models::input::subcommands::Subcommands;
use crate::file_object::FileObject;


fn main() {
    let ctime = SystemTime::now();

    #[cfg_attr(miri, ignore)]
    let options: Config = Parser::parse();
    println!("\nUsing file: {}\n", Path::new(& options.file).file_name().unwrap().to_str().unwrap());
    if !Path::new(&options.file).exists() {
        println!("\nErr: File not found.\n");
        return ()
    }
    rayon::ThreadPoolBuilder::new().num_threads(options.threads).build_global().unwrap();

    let data = Arc::new(FileObject::from_file(&options.file).unwrap());
    
    if options.properties {
        data.print_details();
    }
    if options.column_numbers {
        data.print_column_numbers()
    }
    

    match &options.subcommands {
        // Primary Key subcommand
        Some(Subcommands::PK(opt)) => {
            if opt.calculate_known_keys {
                // let duplicate_row_count = data.get_duplicate_row_count().unwrap_or(0);
                let known_keys = get_known_keys(&data, 0).expect("Could not calculate known keys.");
                println!("\nKnown keys: {:?}", data.header_index_to_string(&known_keys));
                println!("\nPrimary Key Combination: {:?}", get_pk(&data, 0, known_keys).expect("Couldn't find primary key combination."))
            };
            match &opt.output {
                Some(output_location) => get_duplicates_count(&data, &output_location, opt.max_combination_size).expect("Encountered a problem while running the output combination option."),
                None => {
                    if ! opt.calculate_known_keys {
                        // let duplicate_row_count = data.get_duplicate_row_count().unwrap_or(0);
                        println!("\nPrimary Key Combination: {:?}", get_pk(&data, 0, vec![]).expect("Couldn't find primary key combination."));
                    }
                }
            }
        },
        // Column Set subcommand
        Some(Subcommands::ColumnSet(opt)) => {
            if any(&opt.specific_columns, |x| x < &1) {
                println!("All specified columns must be between 1 and {}.", data.total_columns);
                return ();
            }

            // convert to 0-based indexing
            let test_set: Vec<usize> = opt.specific_columns.iter().map(|x| x - 1).collect();
            println!("\nTesting these columns: {:?}", data.header_index_to_string(&test_set));

            // Write duplicates to file if output option given
            match &opt.output {
                Some(output_location) => println!("\nWrote duplicates to file: {:?}\nDuplicates found: {:?}", &output_location, save_columnset_duplicates(&data, &test_set, &output_location).unwrap()),
                None => println!("\nDuplicates found: {:?}", get_columnset_duplicates_count(&data, &test_set).unwrap())
            }            
        },
        None => {
            if !options.properties & !options.column_numbers {
                println!("No commands given, defaulting to finding primary key combination.\n");
                // let duplicate_row_count = data.get_duplicate_row_count().unwrap_or(0);
                println!("\nPrimary Key Combination: {:?}", get_pk(&data, 0, vec![]).expect("Couldn't find primary key combination."));
            };
        }
    }

    if options.debug {
        println!("Total run time: {:?}\n", SystemTime::now().duration_since(ctime).unwrap())
    }
}
