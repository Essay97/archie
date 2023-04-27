use std::{env, fs, path::Path};

use anyhow::{anyhow, Context};

use crate::config::{self, Config};

use super::{Cli, Commands};

pub struct Runner<'a> {
    config: Config,
    command: &'a Commands,
}

impl<'a> Runner<'a> {
    pub fn new(cli: &'a Cli) -> anyhow::Result<Self> {
        let mut config_file = config::get_file_by_priority(&cli.config)?;
        let config = Config::from_file(&mut config_file)?;

        Ok(Self {
            config,
            command: &cli.command,
        })
    }

    pub fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Commands::Build {
                path,
                template,
                name,
            } => self.build(path, template, name),
            Commands::List => self.list(),
            Commands::Info { template } => self.info(template),
        }
    }

    fn build(
        &self,
        path: &Path,
        template_id: &str,
        root_folder_name: &Option<String>,
    ) -> anyhow::Result<()> {
        let template = self
            .config
            .template_by_name(template_id)
            .ok_or(anyhow!("could not find template {}", template_id))?;

        if !crate::path_exists(path)? {
            return Err(anyhow::anyhow!("path {} does not exist", path.display()));
        }

        let base_dir = &path.join(
            root_folder_name
                .as_ref()
                .unwrap_or(&template_id.to_string()),
        );
        if crate::path_exists(base_dir)? {
            return Err(anyhow::anyhow!(
                "root folder {} already exists",
                base_dir.display()
            ));
        }

        fs::create_dir(base_dir)
            .with_context(|| format!("could not create folder {}", base_dir.display()))?;
        env::set_current_dir(base_dir)
            .with_context(|| format!("could not move to folder {}", base_dir.display()))?;

        template.build()?;

        Ok(())
    }

    fn list(&self) -> anyhow::Result<()> {
        println!("TEMPLATES");
        for template in self.config.templates() {
            println!("{}", template.name());
        }
        Ok(())
    }

    fn info(&self, template_id: &str) -> anyhow::Result<()> {
        let template = self
            .config
            .template_by_name(template_id)
            .ok_or(anyhow!("could not find template {}", template_id))?;

        println!("{template_id}");
        template.print();

        Ok(())
    }
}
