use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DataTypes {
    Int,
    Float,
    Bool,
    Colour,
}

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
    // Binary operators
    Multiply,
    Divide,
    And,
    Plus,
    Minus,
    Or,
    // Relational operators
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Equal,
    NotEqual,
    // Types
    Type(DataTypes),
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
            TokenKind::Multiply => write!(f, "Multiply"),
            TokenKind::Divide => write!(f, "Divide"),
            TokenKind::And => write!(f, "And"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Or => write!(f, "Or"),
            TokenKind::LessThan => write!(f, "LessThan"),
            TokenKind::LessThanEqual => write!(f, "LessThanEqual"),
            TokenKind::GreaterThan => write!(f, "GreaterThan"),
            TokenKind::GreaterThanEqual => write!(f, "GreaterThanEqual"),
            TokenKind::Equal => write!(f, "Equal"),
            TokenKind::NotEqual => write!(f, "NotEqual"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Colon => write!(f, "Colon"),
            TokenKind::For => write!(f, "For"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::While => write!(f, "While"),
            TokenKind::Function => write!(f, "Function"),
            TokenKind::Type(DataTypes::Int) => write!(f, "IntType"),
            TokenKind::Type(DataTypes::Float) => write!(f, "FloatType"),
            TokenKind::Type(DataTypes::Bool) => write!(f, "BoolType"),
            TokenKind::Type(DataTypes::Colour) => write!(f, "ColourType"),
            TokenKind::IntLiteral(i) => write!(f, "Int({})", i),
            TokenKind::FloatLiteral(fl) => write!(f, "Float({})", fl),
            TokenKind::BoolLiteral(b) => write!(f, "Bool({})", b),
            TokenKind::ColourLiteral(c) => write!(f, "Colour({:?})", c),
            TokenKind::EndOfFile => write!(f, "EndOfFile"),
            TokenKind::Identifier(s) => write!(f, "Identifier({s})"),
        }
    }
}
