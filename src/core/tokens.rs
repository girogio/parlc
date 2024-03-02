use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Whitespace,
    Invalid,
    Newline,
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    DoubleQuote,
    SingleQuote,
    Semicolon,
    EndOfFile,
    Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    line: usize,
    col: usize,
    lexeme: String,
}

impl TextSpan {
    pub fn new(line: usize, col: usize, lexeme: &str) -> TextSpan {
        TextSpan {
            line,
            col,
            lexeme: lexeme.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Token {
        Token { kind, span }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:?}{}",
            self.kind,
            match self.kind {
                TokenKind::Identifier => format!("({})", self.span.lexeme.clone()),
                _ => "".to_string(),
            }
        )
    }
}
