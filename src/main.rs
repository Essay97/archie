use std::{fs::File, io::BufReader};

use crate::config::parse;

mod config;
mod serde_archie;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("examples/config/config.archie")?;
    let reader = BufReader::new(file);
    let parsed = parse::from_buf_read(reader);

    println!("{parsed:#?}");

    Ok(())
}
