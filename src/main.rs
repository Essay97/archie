use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use std::path::PathBuf;

mod cli;
mod config;
mod error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut config_file = config::get_file_by_priority(&cli.config)?;
    let config = Config::from_file(&mut config_file)?;

    match cli.command {
        Commands::Build {
            name,
            path,
            template,
        } => {
            build(path, template, name, config)?;
        }
    }

    Ok(())
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
