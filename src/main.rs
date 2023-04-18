use std::{fs::File, io::BufReader};

use crate::config::parse;

mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("examples/config/config.archie")?;
    let reader = BufReader::new(file);
    let parsed = parse::from_buf_reader(reader);

    Ok(())
}
