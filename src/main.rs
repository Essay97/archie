use clap::Parser;
use cli::Cli;

mod cli;
mod config;
mod error;

fn main() -> error::Exit {
    let cli = Cli::parse();

    let runner = match cli::get_runner(cli) {
        Ok(r) => r,
        Err(e) => return error::Exit::Err(e),
    };

    runner.run()

    /* match cli.command {
        Commands::Build {
            name,
            path,
            template,
        } => {
            build(path, template, name, config)?;
        }
    } */
}

/* fn build(
    path: PathBuf,
    template_id: String,
    root_folder_name: Option<String>,
    config: Config,
) -> error::Result<()> {
    let template = config
        .template_by_name(&template_id)
        .ok_or(error::Error::TemplateNotFound(template_id))?;

    println!("{template:#?}");

    Ok(())
} */
