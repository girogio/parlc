use std::path::PathBuf;

use thiserror::Error;

use crate::{
    core::{TextSpan, Token, TokenKind},
    semantics::utils::Type,
};

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
    #[error("Unexpected token found at {}:{}:{} \nExpected one of these types: {:?}\nFound: {found}", .source_file.display(), .found.span.from_line, .found.span.from_col, .expected)]
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
    #[error("Variable '{}' is not defined.", .0.span.lexeme)]
    UndefinedVariable(Token),
    #[error("Variable '{}' is already defined.", .0.span.lexeme)]
    VariableRedaclaration(Token),
    #[error("Function '{}' is not defined.", .0.span.lexeme)]
    UndefinedFunction(Token),
    #[error("Function '{}' is already defined.", .0.span.lexeme)]
    AlreadyDefinedFunction(Token),
    // #[error("Variable '{}' is redeclared.", .0.span.lexeme)]
    // RedeclaredVariable(Token),
    #[error("'{}' is of type {:?}, expected {:?}.", .0, .1, .2)]
    TypeMismatch(String, Type, Type),
    #[error("Union type '{}' is of type {:?}, expected one of these types: {:?}.", .0, .1, .2)]
    TypeMismatchUnion(String, Type, Vec<Type>),
    #[error("Invalid operation: {:?}", .0)]
    InvalidOperation(Token),
    #[error("Couldn't cast {:?} to {:?}.", .0, .1)]
    InvalidCast(Type, Type),
    #[error("Function '{}' has a return type of: {:?}, got: {:?}.", .0.span.lexeme, .1, .2)]
    FunctionReturnTypeMismatch(Token, Type, Type),
    #[error("Function '{}' takes no parameters, supplied {:?}.", .0, .1)]
    FunctionCallNoParams(String, Vec<Type>),
}

pub type Result<T> = std::result::Result<T, Error>;
