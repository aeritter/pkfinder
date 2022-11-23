use clap::Parser;
use pyo3::FromPyObject;
use crate::{file_object::FileObject, calculators::DatasetTyper};

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
        let typer = DatasetTyper::new(file_object, self.trim_whitespace);
        if self.details {
            typer.print_detailed_guesses()
        } else {
            typer.print_guesses()
        }
    }
}
