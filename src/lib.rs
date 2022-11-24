#![feature(iter_advance_by)]
mod file_object;
mod common;
mod calculators;
mod models;
use std::sync::Arc;

use crate::calculators::{get_pk, determine_delimiter, get_known_keys};
use crate::file_object::FileObject;
use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "pkfinder")]
fn pkfinder(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyclass(freelist = 1000)]
    #[derive(Clone)]
    struct PKFinder {
        file_object: Arc<FileObject>
    }

    #[pymethods]
    impl PKFinder {
        // Loads the file into memory so it can be worked on.
        #[new]
        fn new(
            from_file: Option<&str>,
            delimiter: Option<&str>
        ) -> PyResult<Self> {
            let from_file = from_file.expect("Must provide path to file");
            let delim = determine_delimiter(&from_file).unwrap();
            let delimiter = match delimiter {
                Some(x) => x,
                None => &delim
            };
            let fobject = Arc::new(FileObject::from_file(&from_file, &delimiter, true, 0, false).unwrap());
            return Ok(PKFinder { file_object: fobject })
        }

        // Returns a combination with the fewest possible columns that result in no duplicates that could be used as a primary key combination for the dataset.
        fn get_pk(&self, known_keys: Option<Vec<usize>>) -> PyResult<Vec<String>> {
            let keylist = match known_keys {
                Some(x) => x,
                None => vec![]  
            };
            return Ok(get_pk(&self.file_object, 0, keylist).unwrap())
        }

        fn get_known_keys(&self) -> PyResult<Vec<String>> {
            return Ok(self.file_object.header_index_to_string(&get_known_keys(&self.file_object, 0).unwrap()))
        }

    }

    // m.add_function(wrap_pyfunction!(find_pk, m)?)?;
    m.add_class::<PKFinder>()?;
    Ok(())
}