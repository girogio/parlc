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
    #[error("Parse error in file: {file}:{line}:{col}\nUnexpected token found at {}:{}:{} \nExpected {expected:?}, found {found}", .source_file.display(), .found.span.from_line, .found.span.from_col)]
    UnexpectedToken {
        expected: TokenKind,
        found: Token,
        source_file: PathBuf,
        file: &'static str,
        line: u32,
        col: u32,
    },
    #[error("Parser error in file: {}:{}:{}\nUnexpected token found at {}:{}:{} \nExpected one of these types: {:?}",.file, .line, .col, .source_file.display(), .found.span.from_line, .found.span.from_col, .expected)]
    UnexpectedTokenList {
        file: &'static str,
        line: u32,
        col: u32,
        source_file: PathBuf,
        found: Token,
        expected: Vec<TokenKind>,
    },
    #[error("Unclosed block.")]
    UnclosedBlock,
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::TokenKind;

    #[test]
    fn test_lexical_error() {
        let error = LexicalError::InvalidCharacter(TextSpan::new(1, 1, 1, 1, "a"));
        assert_eq!(
            format!("{}", error),
            "Unrecognized character 'a' found at 1:1"
        );

        let error = LexicalError::UnterminatedString(TextSpan::new(1, 1, 1, 1, "a"));
        assert_eq!(format!("{}", error), "Unterminated string.");

        let error = LexicalError::InvalidFloatLiteral(TextSpan::new(1, 1, 1, 1, "a"));
        assert_eq!(format!("{}", error), "Invalid float literal ending at 1:1");

        let error = LexicalError::UnterminatedBlockComment(TextSpan::new(1, 1, 1, 1, "a"));
        assert_eq!(format!("{}", error), "Unterminated block comment.");
    }

    #[test]
    fn test_parse_error() {
        let error = ParseError::UnexpectedToken {
            expected: TokenKind::Identifier,
            found: Token::new(TokenKind::IntLiteral, TextSpan::new(1, 1, 1, 1, "1")),
            file: "file",
            source_file: PathBuf::from("source_file"),
            line: 1,
            col: 1,
        };
        assert_eq!(
            format!("{}", error),
            "Parse error in file: file:1:1\nUnexpected token: expected Identifier, found Token { kind: IntLiteral, lexeme: \"1\", span: TextSpan { from_line: 1, from_col: 1, to_line: 1, to_col: 1 } }"
        );

        let error = ParseError::UnexpectedTokenList {
            file: "file",
            source_file: "source_file",
            line: 1,
            col: 1,
            found: Token::new(TokenKind::IntLiteral, TextSpan::new(1, 1, 1, 1, "1")),
            expected: vec![TokenKind::Identifier, TokenKind::FloatLiteral],
        };
    }
}
