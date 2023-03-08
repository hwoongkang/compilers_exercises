#[derive(Debug, PartialEq, Eq)]
pub enum LexerError {
    UnexpectedToken,
    UnexpectedEOF,
}
