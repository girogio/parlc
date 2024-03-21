#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
pub enum TokenKind {
    Colon,
    Comma,
    Comment,
    EndOfFile,
    Equals,
    Identifier,
    Invalid,
    LBrace,
    LBracket,
    LParen,
    Newline,
    RBrace,
    RBracket,
    RParen,
    Semicolon,
    Whitespace,
    // Keywords
    As,
    Delay,
    Else,
    For,
    Function,
    If,
    Let,
    PadClear,
    PadHeight,
    PadRandI,
    PadRead,
    PadWidth,
    PadWrite,
    PadWriteBox,
    Print,
    Return,
    While,
    // Binary operators
    And,
    Divide,
    Minus,
    Multiply,
    Or,
    Not,
    Plus,
    // Relational operators
    EqEq,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    NotEqual,
    // Type and literal stuff
    BoolLiteral,
    ColourLiteral,
    FloatLiteral,
    IntLiteral,
    Type,
    Arrow,
}
