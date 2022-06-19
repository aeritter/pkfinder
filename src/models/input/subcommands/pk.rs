use clap::Parser;

#[derive(Debug, Parser)]
pub struct PKOptions {
    /// Calculate known keys first. Faster if the result has many primary keys.
    #[clap(short)]
    pub calculate_known_keys: bool,
    /// Calculate duplicate count for every combination up to max_combination_size and write to the specified file.
    #[clap(short, long = "output_path")]
    pub output: Option<String>,
    /// Set max combination size for duplicate count (-o option)
    #[clap(short, long, default_value = "5")]
    pub max_combination_size: usize,
}