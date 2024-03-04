use thiserror::Error;

use crate::core::{TextSpan, Token};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Lexical error: {0}")]
    Lexical(#[from] LexicalError),
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
}

#[derive(Debug, Error)]
pub enum LexicalError {
    #[error("Unrecognized character '{}' found at {}:{}", .0.lexeme, .0.from_line, .0.from_col)]
    InvalidCharacter(TextSpan),
    #[error("Unterminated string.")]
    UnterminatedString(TextSpan),
    #[error("Invalid float literal ending at {}:{}", .0.to_line, .0.to_col)]
    InvalidFloatLiteral(TextSpan),
    #[error("Unterminated block comment.")]
    UnterminatedBlockComment(TextSpan),
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unexpected token: {0:?}")]
    UnexpectedToken(Token),
}

// pub type Result<T> = std::result::Result<T, Error>;
