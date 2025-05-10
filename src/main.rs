mod file_object;
mod common;
mod calculators;
mod models;
use std::path::Path;
use std::sync::Arc;
use std::time::SystemTime;

use clap::Parser;
use crate::calculators::{get_pk, determine_delimiter};
use crate::common::{print_details, print_column_numbers};
use crate::models::input::Config;
use crate::models::input::subcommands::Subcommands;
use crate::file_object::FileObject;


fn main() {
    let ctime = SystemTime::now();

    let options: Config = Parser::parse();
    println!("\nUsing file: {}\n", Path::new(& options.file).file_name().unwrap().to_str().unwrap());
    if !Path::new(&options.file).exists() {
        println!("\nErr: File not found.\n");
        return ()
    }
    rayon::ThreadPoolBuilder::new().num_threads(options.threads).build_global().unwrap();

    let delimiter: String;
    match &options.delimiter {
        Some(x) => {delimiter = x.to_string();},
        None => {
            delimiter = determine_delimiter(&options.file, &options.skip).expect("Failed while attempting to find delimiter");
        }
    }
    
    if options.properties {
        print_details(&options.file, &delimiter).expect("Expected to read file and print details");
    }
    if options.column_numbers {
        print_column_numbers(&options.file, &delimiter, options.skip).expect("Expected to read file and print columns");
    }
    

    match &options.subcommands {
        // Primary Key subcommand
        Some(Subcommands::PK(opt)) => {
            opt.run(&Arc::new(FileObject::from_file(&options.file, &delimiter, true, options.skip, options.debug).unwrap()));
        },
        // Column Set subcommand
        Some(Subcommands::ColumnSet(opt)) => {
            opt.run(&Arc::new(FileObject::from_file(&options.file, &delimiter, true, options.skip, options.debug).unwrap()));         
        },
        // Typer subcommand
        Some(Subcommands::Typer(opt)) => {
            opt.run(&Arc::new(FileObject::from_file(&options.file, &delimiter, false, options.skip, options.debug).unwrap()));
        },
        None => {
            if !options.properties & !options.column_numbers {
                println!("No commands given, defaulting to finding primary key combination.\n");
                // let duplicate_row_count = data.get_duplicate_row_count().unwrap_or(0);
                println!("\nPrimary Key Combination: {:?}", get_pk(&Arc::new(FileObject::from_file(&options.file, &delimiter, true, options.skip, options.debug).unwrap()), 0, vec![]).expect("Couldn't find primary key combination."));
            };
        }
    }

    if options.debug {
        println!("\nTotal run time: {:?}\n", SystemTime::now().duration_since(ctime).unwrap())
    }
    println!()
}
