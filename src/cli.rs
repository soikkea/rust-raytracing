use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Output file
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>
}