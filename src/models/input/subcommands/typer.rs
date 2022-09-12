use clap::Parser;
use pyo3::FromPyObject;
use crate::{file_object::FileObject, calculators::get_types};

#[derive(Debug, Parser, FromPyObject)]
pub struct TyperOptions {
    /// Get detailed stats
    #[clap(short, long)]
    pub details: bool,
}

impl TyperOptions {
    pub fn run(&self, file_object: &FileObject) {
        get_types(file_object, self.details);
    }
}
