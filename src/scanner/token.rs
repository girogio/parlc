use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Whitespace,
    Invalid,
    Register,
    EndOfFile,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    start: usize,
    end: usize,
    lexeme: String,
}

impl TextSpan {
    pub fn new(start: usize, end: usize, lexeme: &str) -> TextSpan {
        TextSpan {
            start,
            end,
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
                TokenKind::EndOfFile => "".to_string(),
                TokenKind::Whitespace => "".to_string(),
                _ => format!("({})", self.span.lexeme.clone()),
            }
        )
    }
}
