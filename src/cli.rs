use std::path::PathBuf;

use clap::Parser;

use crate::scenes::Scene;

#[derive(Debug, Parser)]
#[command(about, long_about = None)]
pub struct Cli {
    /// Use without GUI
    #[arg(long, requires_all = ["output", "scene"])]
    pub no_gui: bool,

    /// Output file
    #[arg(short, long, value_name = "FILE", requires = "no_gui")]
    pub output: Option<PathBuf>,

    /// Scene to render
    #[arg(short, long, requires = "no_gui")]
    pub scene: Option<Scene>,
}
