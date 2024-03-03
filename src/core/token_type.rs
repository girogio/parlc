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
    SingleQuote,
    StringLiteral(String),
    Identifier(String),
    Semicolon,
    EndOfFile,
    Colon,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Invalid => write!(f, "Invalid"),
            TokenKind::Newline => write!(f, "Newline"),
            TokenKind::LBrace => write!(f, "LBrace"),
            TokenKind::RBrace => write!(f, "RBrace"),
            TokenKind::LParen => write!(f, "LParen"),
            TokenKind::RParen => write!(f, "RParen"),
            TokenKind::LBracket => write!(f, "LBracket"),
            TokenKind::RBracket => write!(f, "RBracket"),
            TokenKind::SingleQuote => write!(f, "SingleQuote"),
            TokenKind::StringLiteral(s) => write!(f, "StringLiteral({s})"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Colon => write!(f, "Colon"),
            TokenKind::EndOfFile => write!(f, "EndOfFile"),
            TokenKind::Identifier(s) => write!(f, "Identifier({s})"),
        }
    }
}
