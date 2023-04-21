use crate::{
    config::{self, Config},
    error,
};

use super::Cli;

pub struct Runner {
    config: Config,
    debug: bool,
}

impl Runner {
    pub fn new(cli: Cli) -> error::Result<Self> {
        let mut config_file = config::get_file_by_priority(&cli.config)?;
        let config = Config::from_file(&mut config_file)?;

        Ok(Self {
            config,
            debug: true,
        })
    }
}
