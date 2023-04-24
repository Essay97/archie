use std::{env, fs, path::Path};

use crate::{
    config::{self, Config},
    error,
};

use super::{Cli, Commands};

pub struct Runner<'a> {
    config: Config,
    command: &'a Commands,
}

impl<'a> Runner<'a> {
    pub fn new(cli: &'a Cli) -> error::Result<Self> {
        let mut config_file = config::get_file_by_priority(&cli.config)?;
        let config = Config::from_file(&mut config_file)?;

        Ok(Self {
            config,
            command: &cli.command,
        })
    }

    pub fn run(&self) -> error::Exit {
        match &self.command {
            Commands::Build {
                path,
                template,
                name,
            } => self.build(path, template, name).into(),
            Commands::List => self.list().into(),
            Commands::Info { template } => self.info(template).into(),
        }
    }

    fn build(
        &self,
        path: &Path,
        template_id: &str,
        root_folder_name: &Option<String>,
    ) -> error::Result<()> {
        let template = self
            .config
            .template_by_name(template_id)
            .ok_or(error::Error::TemplateNotFound(template_id.to_owned()))?;

        if !crate::path_exists(path)? {
            return Err(error::Error::PathNotExistent(path.to_path_buf()));
        }

        let base_dir = &path.join(
            root_folder_name
                .as_ref()
                .unwrap_or(&template_id.to_string()),
        );
        if crate::path_exists(base_dir)? {
            return Err(error::Error::RootFolderExistent(base_dir.to_owned()));
        }

        fs::create_dir(base_dir).map_err(|_| error::Error::OnCreateFolder(base_dir.to_owned()))?;
        env::set_current_dir(base_dir)
            .map_err(|_| error::Error::OnChangeFolder(base_dir.to_owned()))?;

        template.build()?;

        Ok(())
    }

    fn list(&self) -> error::Result<()> {
        for template in self.config.templates() {
            println!("{}", template.name());
        }
        Ok(())
    }

    fn info(&self, template_id: &str) -> error::Result<()> {
        let template = self
            .config
            .template_by_name(template_id)
            .ok_or(error::Error::TemplateNotFound(template_id.to_owned()))?;

        println!("{template_id}");
        template.print();

        Ok(())
    }
}
