use std::io::BufRead;

#[derive(Debug)]
pub enum Error {
    /// An header is opened with \[ but not closed with \] in the same line
    NoClosingHeader,
    /// An IO error happened while reading the line
    IO,
    /// A template does not begin with a header
    MissingHeader,
    /// Found a header in an already started template
    HeaderInTemplate,
}

#[derive(Debug)]
enum ConfigLineKind {
    Header,
    Folder,
    File,
    Empty,
}

#[derive(Debug)]
pub struct ConfigLine {
    indent: u32,
    identifier: String,
    kind: ConfigLineKind,
}

impl TryFrom<&str> for ConfigLine {
    type Error = Error;

    fn try_from(line: &str) -> Result<Self, Error> {
        let mut kind = ConfigLineKind::File; // just initialization, could be everything
        let mut started_identifier = false;
        let mut closed_header = false;
        let mut indent_count: u32 = 0;
        let mut identifier = String::new();

        if line.trim().len() == 0 {
            return Ok(Self::empty());
        }

        for c in line.chars() {
            if c == '[' {
                started_identifier = true;
                kind = ConfigLineKind::Header
            } else if c == ' ' && !started_identifier {
                indent_count += 1;
            } else if (c == '/' || c == ']') && started_identifier {
                if c == '/' {
                    kind = ConfigLineKind::Folder;
                } else {
                    closed_header = true;
                }
                break;
            } else {
                if !started_identifier {
                    started_identifier = true;
                }
                identifier.push(c);
            }
        }

        if matches!(kind, ConfigLineKind::Header) && started_identifier && !closed_header {
            println!("{identifier}");
            return Err(Error::NoClosingHeader);
        }

        Ok(Self {
            indent: indent_count / 2,
            identifier: identifier,
            kind: kind,
        })
    }
}

impl ConfigLine {
    fn empty() -> Self {
        Self {
            indent: 0,
            identifier: String::new(),
            kind: ConfigLineKind::Empty,
        }
    }
}

pub fn from_buf_read<R: BufRead>(reader: R) -> Vec<Result<ConfigLine, Error>> {
    let lines = reader.lines();

    let mut config_lines: Vec<Result<ConfigLine, Error>> = Vec::new();

    for line in lines {
        match line {
            Ok(val) => config_lines.push(ConfigLine::try_from(&val[..])),
            Err(_) => config_lines.push(Err(Error::IO)),
        }
    }

    config_lines
}

fn check_syntax(lines: Vec<ConfigLine>) -> Result<(), Error> {
    let mut template_started = false;

    for line in lines {
        if !template_started {
            // Check if template starts with header
            if !matches!(line.kind, ConfigLineKind::Header) {
                return Err(Error::MissingHeader);
            } else {
                template_started = true;
            }
        } else {
            // Check that template contains only nodes
            match line.kind {
                ConfigLineKind::File | ConfigLineKind::Folder => continue,
                ConfigLineKind::Header => return Err(Error::HeaderInTemplate),
                ConfigLineKind::Empty => template_started = false,
            }
        }
    }

    Ok(())
}
