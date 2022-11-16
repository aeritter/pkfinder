use clap::Parser;
use pyo3::FromPyObject;
use crate::{file_object::FileObject, calculators::get_types};

#[derive(Debug, Parser, FromPyObject)]
pub struct TyperOptions {
    /// Get detailed stats
    #[clap(short, long)]
    pub details: bool,

    /// Trim whitespace from each field before determining types
    #[clap(short, long)]
    pub trim_whitespace: bool,
}

impl TyperOptions {
    pub fn run(&self, file_object: &FileObject) {
        get_types(file_object, self.details, self.trim_whitespace);
    }
}
