use std::path::PathBuf;

use crate::{
    config::{self, Config},
    error,
};

use super::{Cli, Commands};

pub struct Runner {
    config: Config,
    debug: bool,
    command: Commands,
}

impl Runner {
    pub fn new(cli: Cli) -> error::Result<Self> {
        let mut config_file = config::get_file_by_priority(&cli.config)?;
        let config = Config::from_file(&mut config_file)?;

        Ok(Self {
            config,
            debug: cli.debug,
            command: cli.command,
        })
    }

    pub fn run(&self) -> error::Exit {
        match &self.command {
            Commands::Build {
                path,
                template,
                name,
            } => error::Exit::from(self.build(path, template, name), self.debug),
        }
    }

    fn build(
        &self,
        path: &PathBuf,
        template_id: &String,
        root_folder_name: &Option<String>,
    ) -> error::Result<()> {
        let template = self
            .config
            .template_by_name(template_id)
            .ok_or(error::Error::TemplateNotFound(template_id.to_owned()))?;

        println!("{template:#?}");

        Ok(())
    }
}
