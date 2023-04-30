use clap::{Parser, Subcommand};
use std::path::PathBuf;

use runner::Runner;

pub mod runner;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Specify a configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build the given template
    Build {
        /// The path where to build the template
        path: PathBuf,
        /// The identifier of the template to build
        template: Option<String>,
        #[arg(short, long)]
        /// Set name of the root folder (defaults to template name)
        name: Option<String>,
    },
    /// List all the templates available
    List,
    /// See the structure of a template
    Info {
        /// The template for which you want to see the structure
        template: String,
    },
    /// All debug purposes
    Debug,
}

pub fn get_runner(cli: &Cli) -> anyhow::Result<Runner> {
    Runner::new(cli)
}
