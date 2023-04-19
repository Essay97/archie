use clap::Parser;
use cli::{Cli, Commands};
use config::{ConfigData, Template};
use std::fs;

mod cli;
mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* let config_file = fs::read_to_string("examples/config/.archierc.yaml")?;
    let config: ConfigData = serde_yaml::from_str(&config_file)?;

    match config.templates.get("template1") {
        None => panic!("template not found"),
        Some(template) => {
            let x = Template::from_template_data("template1", template);
            println!("{x:#?}");
        }
    }

    println!("Done"); */

    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            path,
            template,
            name,
        } => {
            build(path, template, name);
        }
    }

    Ok(())
}

fn build(path: String, template_id: String, root_folder_name: Option<String>) {}
