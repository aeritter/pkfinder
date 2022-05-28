mod file_object;
mod common;
mod calculators;
use std::path::Path;
use std::sync::Arc;
use std::time::SystemTime;
use itertools::any;
use clap::Parser;
use crate::calculators::{get_columnset_duplicates_count, save_columnset_duplicates, get_pk, get_known_keys, get_duplicates_count};
use crate::file_object::FileObject;



#[derive(Debug, Parser)]
#[clap(name = "Primary Key finder", about = "Assists with finding the best primary key combination for a given dataset.", author = "by Andrew Ritter")]
struct Config {
    /// Location of the file to process
    file: String,

    /// Print file details (delimiter, column & row counts)
    #[clap(short, long)]
    properties: bool,
    
    /// Number of CPU threads to use. Defaults to two (2) threads.
    #[clap(short, long, default_value = "2")]
    threads: usize,

    /// Print column numbers and the associated headers
    #[clap(short, long)]
    column_numbers: bool,

    /// Debugging (only displays total run time at the moment)
    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    subcommands: Option<Subcommands>
}


#[derive(Debug, Parser)]
enum Subcommands {
    /// Use pk to find the smallest possible primary key combination.
    /// 
    /// WARNING: The Primary Key combination is calculated by finding the smallest combination of columns that result in as few duplicates as possible. You will have to look at the data itself and understand what the data means to determine if the result given needs to be adjusted.
    #[clap(name = "pk", verbatim_doc_comment)]
    PK(PKOptions),
    #[clap(name = "cs", about = "Use cs to check a specific columnset")]
    ColumnSet(ColumnSetOptions)
}


#[derive(Debug, Parser)]
struct PKOptions {
    /// Calculate known keys first. Faster if the result has many primary keys.
    #[clap(short)]
    calculate_known_keys: bool,
    /// Calculate duplicate count for every combination up to max_combination_size and write to the specified file.
    #[clap(short, long = "output_path")]
    output: Option<String>,
    /// Set max combination size for duplicate count (-o option)
    #[clap(short, long, default_value = "5")]
    max_combination_size: usize,
}

#[derive(Debug, Parser)]
struct ColumnSetOptions {
    /// Find duplicates for the given set of column numbers separated by spaces.
    /// Example: -s 1 3 4 8
    specific_columns: Vec<usize>,
    /// Output list of duplicates found to the specified file.
    #[clap(short, long = "output_path")]
    output: Option<String>,
}



// #[derive(Debug, Parser)]
// struct Output {
//     #[clap(short, long = "output_path")]
//     outfile: Option<String>,
// }


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

    let data = Arc::new(FileObject::new(&options.file).unwrap());
    
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
            println!("No commands given, defaulting to finding primary key combination.\n");
            // let duplicate_row_count = data.get_duplicate_row_count().unwrap_or(0);
            println!("\nPrimary Key Combination: {:?}", get_pk(&data, 0, vec![]).expect("Couldn't find primary key combination."));
        }
    }

    if options.debug {
        println!("Total run time: {:?}\n", SystemTime::now().duration_since(ctime).unwrap())
    }
}
