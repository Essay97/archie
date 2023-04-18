use super::line::LineErrorKind;

struct ParseError {
    line_number: u32,
    line: String,
    kind: LineErrorKind,
}
