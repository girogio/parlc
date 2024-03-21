use std::path::PathBuf;

use thiserror::Error;

use crate::core::{TextSpan, Token, TokenKind};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Lexical error: {0}")]
    Lexical(#[from] LexicalError),
    #[error("Parse error: {0}")]
    Parse(#[from] ParseError),
    #[error("Semantic error: {0}")]
    Semantic(#[from] SemanticError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum LexicalError {
    #[error("Unrecognized character '{}' found at {}:{}", .0.lexeme, .0.from_line, .0.from_col)]
    InvalidCharacter(TextSpan),
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

#[derive(Debug, Error)]
pub enum SemanticError {
    #[error("Variable '{0}' is not defined.")]
    UndefinedVariable(String),
    #[error("Variable '{0}' is already defined.")]
    AlreadyDefinedVariable(String),
    #[error("Function '{0}' is not defined.")]
    UndefinedFunction(String),
    #[error("Function '{0}' is already defined.")]
    AlreadyDefinedFunction(String),
}

pub type Result<T> = std::result::Result<T, Error>;
