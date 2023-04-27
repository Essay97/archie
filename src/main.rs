use clap::Parser;
use cli::Cli;

mod cli;
mod config;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let runner = cli::get_runner(&cli)?;

    runner.run()
}
