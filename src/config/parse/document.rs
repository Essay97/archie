use super::{super::Config, error::ParseError, line::ConfigLine};

pub fn parse(document: Vec<ConfigLine>) -> Result<Config, Vec<ParseError>> {}
