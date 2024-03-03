use thiserror::Error;

use crate::models::TextSpan;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Lexical error: {0}")]
    Lexical(#[from] LexicalError),
}

#[derive(Debug, Error)]
pub enum LexicalError {
    #[error("Invalid character '{}' found at position {}:{}", .0.lexeme, .0.from_line, .0.from_col)]
    InvalidCharacter(TextSpan),
    #[error("Unterminated string.")]
    UnterminatedString(TextSpan),
}

pub type Result<T> = std::result::Result<T, Error>;
