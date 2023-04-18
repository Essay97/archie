use super::line::LineErrorKind;

pub struct ParseError {
    line_number: u32,
    line: String,
    kind: LineErrorKind,
}
