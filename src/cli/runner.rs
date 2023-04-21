use std::{env, fs, path::Path};

use crate::{
    config::{self, Config},
    error,
};

use super::{Cli, Commands};

pub struct Runner {
    config: Config,
    command: Commands,
}

impl Runner {
    pub fn new(cli: Cli) -> error::Result<Self> {
        let mut config_file = config::get_file_by_priority(&cli.config)?;
        let config = Config::from_file(&mut config_file)?;

        Ok(Self {
            config,
            command: cli.command,
        })
    }

    pub fn run(&self) -> error::Exit {
        match &self.command {
            Commands::Build {
                path,
                template,
                name,
            } => self.build(path, template, name).into(),
        }
    }

    fn build(
        &self,
        path: &Path,
        template_id: &String,
        root_folder_name: &Option<String>,
    ) -> error::Result<()> {
        let template = self
            .config
            .template_by_name(template_id)
            .ok_or(error::Error::TemplateNotFound(template_id.to_owned()))?;

        path.try_exists()?;

        let base_dir = path.join(root_folder_name.as_ref().unwrap_or(template_id));
        if base_dir.exists() {
            return Err(error::Error::RootFolderExistent(base_dir));
        }

        Ok(())
    }
}
