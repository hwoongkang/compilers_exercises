#[derive(Debug, PartialEq, Eq)]
pub enum LexerError {
    SyntaxError,
    UnexpectedToken,
    UnexpectedEOF,
}
