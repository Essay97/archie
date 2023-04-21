use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::error;

use self::runner::Runner;

pub mod runner;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    /// Specify a config file
    #[arg(short, long)]
    pub config: Option<PathBuf>,
    /// Show debug logs
    #[arg(long)]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Build the given template
    Build {
        /// The path where to build the template
        path: PathBuf,
        /// The identifier of the template to build
        template: String,
        #[arg(short, long)]
        /// Set name of the root folder (defaults to template name)
        name: Option<String>,
    },
}

pub fn get_runner(cli: Cli) -> error::Result<Runner> {
    Runner::new(cli)
}
