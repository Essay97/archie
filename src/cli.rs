use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[arg(short, long)]
    /// Specify a config file
    pub config: Option<PathBuf>,
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
