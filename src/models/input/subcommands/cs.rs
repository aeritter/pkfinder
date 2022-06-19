use clap::Parser;

#[derive(Debug, Parser)]
pub struct ColumnSetOptions {
    /// Find duplicates for the given set of column numbers separated by spaces.
    /// Example: -s 1 3 4 8
    pub specific_columns: Vec<usize>,
    /// Output list of duplicates found to the specified file.
    #[clap(short, long = "output_path")]
    pub output: Option<String>,
}
