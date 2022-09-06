mod file_object;
mod common;
mod calculators;
mod models;
use std::path::Path;
use std::sync::Arc;
use std::time::SystemTime;

use clap::Parser;
use crate::calculators::{get_pk};
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
            opt.run(&data);
        },
        // Column Set subcommand
        Some(Subcommands::ColumnSet(opt)) => {
            opt.run(&data);         
        },
        // Typer subcommand
        Some(Subcommands::Typer(opt)) => {
            opt.run(&data);
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
