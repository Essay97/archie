use std::path::Path;

use clap::Parser;
use cli::Cli;

mod cli;
mod config;
mod error;

fn main() -> error::Exit {
    let cli = Cli::parse();

    let runner = match cli::get_runner(&cli) {
        Ok(r) => r,
        Err(e) => return error::Exit::Err(e),
    };

    runner.run()
}

fn path_exists(path: &Path) -> error::Result<bool> {
    path.try_exists()
        .map_err(|_| error::Error::FileNotAccessible(path.to_path_buf()))
}
