#![feature(try_trait_v2)]
#![feature(never_type)]

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use std::path::PathBuf;

mod cli;
mod config;
mod error;

fn main() -> error::Exit {
    let cli = Cli::parse();

    let mut config_file = config::get_file_by_priority(&cli.config)?;
    let config = Config::from_file(&mut config_file);

    let runner = cli::get_runner(cli);

    /* match cli.command {
        Commands::Build {
            name,
            path,
            template,
        } => {
            build(path, template, name, config)?;
        }
    } */

    error::Exit::Ok
}

fn build(
    path: PathBuf,
    template_id: String,
    root_folder_name: Option<String>,
    config: Config,
) -> Result<(), error::Error> {
    let template = config
        .template_by_name(&template_id)
        .ok_or(error::Error::TemplateNotFound(template_id))?;

    println!("{template:#?}");

    Ok(())
}
