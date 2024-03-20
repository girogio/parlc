use std::path::PathBuf;

use thiserror::Error;

use crate::core::{TextSpan, Token, TokenKind};

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
    #[error("Unexpected token found at {}:{}:{} \nExpected {expected:?}, found {found}", .source_file.display(), .found.span.from_line, .found.span.from_col)]
    UnexpectedToken {
        expected: TokenKind,
        found: Token,
        source_file: PathBuf,
    },
    #[error("Unexpected token found at {}:{}:{} \nExpected one of these types: {:?}", .source_file.display(), .found.span.from_line, .found.span.from_col, .expected)]
    UnexpectedTokenList {
        source_file: PathBuf,
        found: Token,
        expected: Vec<TokenKind>,
    },
    #[error("Unclosed block.")]
    UnclosedBlock,
}

pub type Result<T> = std::result::Result<T, Error>;
