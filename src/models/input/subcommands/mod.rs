use clap::Parser;

mod pk;
use pk::PKOptions;

mod cs;
use cs::ColumnSetOptions;

#[derive(Debug, Parser)]
pub enum Subcommands {
    /// Use pk to find the smallest possible primary key combination.
    /// 
    /// WARNING: The Primary Key combination is calculated by finding the smallest combination of columns that result in as few duplicates as possible. You will have to look at the data itself and understand what the data means to determine if the result given needs to be adjusted.
    #[clap(name = "pk", verbatim_doc_comment)]
    PK(PKOptions),
    #[clap(name = "cs", about = "Use cs to check a specific columnset")]
    ColumnSet(ColumnSetOptions)
}