mod file_object;
mod common;
mod calculators;
mod models;
use std::path::Path;
use std::sync::Arc;
use std::time::SystemTime;
use pyo3::exceptions::PyFileNotFoundError;
use pyo3::types::PyList;

use crate::calculators::{get_pk, determine_delimiter};
use crate::common::{print_details, print_column_numbers};
use crate::models::input::Config;
use crate::models::input::subcommands::Subcommands;
use crate::file_object::FileObject;
use pyo3::prelude::*;

#[pyfunction]
fn find_pk(file: &str) -> PyResult<Vec<String>> {
    let ctime = SystemTime::now();
    println!("{:?}", file);
    
    println!("\nUsing file: {}\n", Path::new(&file).file_name().unwrap().to_str().unwrap());
    if !Path::new(&file).exists() {
        println!("\nErr: File not found.\n");
        return Err(PyFileNotFoundError::new_err("File not found"))
    }
    rayon::ThreadPoolBuilder::new().num_threads(2).build_global().unwrap();

    
    let delimiter = determine_delimiter(&file).expect("Failed while attempting to find delimiter");
    
    let answer = get_pk(&Arc::new(FileObject::from_file(&file, &delimiter, true).unwrap()), 0, vec![]).expect("Couldn't find primary key combination.");

    return Ok(answer)

}

#[pymodule]
#[pyo3(name = "pkfinder")]
fn pkfinder(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(find_pk, m)?)?;
    Ok(())
}