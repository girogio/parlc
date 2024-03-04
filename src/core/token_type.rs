use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
    Whitespace,
    Invalid,
    Newline,
    LBrace,
    RBrace,
    Equals,
    LParen,
    RParen,
    LBracket,
    RBracket,
    SingleQuote,
    StringLiteral(String),
    Identifier(String),
    For,
    If,
    Else,
    While,
    Function,
    Semicolon,
    // Types
    IntType,
    FloatType,
    BoolType,
    ColourType,
    // Type literals
    IntLiteral(i32),
    FloatLiteral(String),
    BoolLiteral(bool),
    ColourLiteral([u8; 3]),
    EndOfFile,
    Colon,
    Comment,
}

// macro that takes in a lis tof tokens and returns the below match statement
impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Invalid => write!(f, "Invalid"),
            TokenKind::Newline => write!(f, "Newline"),
            TokenKind::Comment => write!(f, "Comment"),
            TokenKind::Equals => write!(f, "Equals"),
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
            TokenKind::For => write!(f, "For"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::While => write!(f, "While"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::IntType => write!(f, "IntType"),
            TokenKind::FloatType => write!(f, "FloatType"),
            TokenKind::BoolType => write!(f, "BoolType"),
            TokenKind::ColourType => write!(f, "ColourType"),
            TokenKind::IntLiteral(i) => write!(f, "Int({})", i),
            TokenKind::FloatLiteral(fl) => write!(f, "Float({})", fl),
            TokenKind::BoolLiteral(b) => write!(f, "Bool({})", b),
            TokenKind::ColourLiteral(c) => write!(f, "Colour({:?})", c),
            TokenKind::EndOfFile => write!(f, "EndOfFile"),
            TokenKind::Identifier(s) => write!(f, "Identifier({s})"),
        }
    }
}
