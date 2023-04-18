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

#[derive(Debug)]
pub enum LineErrorKind {
    /// An header is opened with \[ but not closed with \] in the same line
    NoClosingHeader,
    /// An IO error happened while reading the line
    IO,
    /// A template does not begin with a header
    MissingHeader,
    /// Found a header in an already started template
    HeaderInTemplate,
    /// Line is indented with tabs
    WrongIndentation,
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

impl TryFrom<&str> for ConfigLine {
    type Error = LineErrorKind;

    fn try_from(line: &str) -> Result<Self, LineErrorKind> {
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
            } else if c == '\t' && !started_identifier {
                return Err(LineErrorKind::WrongIndentation);
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
            return Err(LineErrorKind::NoClosingHeader);
        }

        Ok(Self {
            indent: indent_count / 2,
            identifier: identifier,
            kind: kind,
        })
    }
}
