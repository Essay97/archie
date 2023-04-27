use std::path::Path;

use anyhow::Context;
use clap::Parser;
use cli::Cli;

mod cli;
mod config;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let runner = cli::get_runner(&cli)?;

    runner.run()
}

fn path_exists(path: &Path) -> anyhow::Result<bool> {
    path.try_exists()
        .with_context(|| format!("could not open file {}", path.display()))
}
