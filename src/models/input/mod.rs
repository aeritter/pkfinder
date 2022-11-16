use clap::Parser;
use pyo3::FromPyObject;

pub mod subcommands;
use self::subcommands::Subcommands;

#[derive(Debug, Parser, FromPyObject)]
#[clap(name = "Primary Key finder", about, version, author)]
pub struct Config {
    /// Location of the file to process
    pub file: String,

    /// Number of lines at top of file to skip (intended for skipping non-csv data in a csv file)
    #[clap(short, long, default_value = "0")]
    pub skip: usize,

    /// Print file details (delimiter, column & row counts)
    #[clap(short, long)]
    pub properties: bool,
    
    /// Number of CPU threads to use.
    #[clap(short, long, default_value = "2")]
    pub threads: usize,

    /// Print column numbers and the associated headers
    #[clap(short, long)]
    pub column_numbers: bool,

    /// Debugging (only displays total run time at the moment)
    #[clap(short, long)]
    pub debug: bool,

    #[clap(subcommand)]
    pub subcommands: Option<Subcommands>
}
