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
            TokenKind::EndOfFile => write!(f, "EndOfFile"),
            TokenKind::Identifier(s) => write!(f, "Identifier({s})"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    from_line: usize,
    to_line: usize,
    from_col: usize,
    to_col: usize,
    pub lexeme: String,
}

impl TextSpan {
    pub fn new(
        from_line: usize,
        to_line: usize,
        from_col: usize,
        to_col: usize,
        lexeme: &str,
    ) -> TextSpan {
        TextSpan {
            from_line,
            to_line,
            from_col,
            to_col,
            lexeme: lexeme.to_string(),
        }
    }
}

impl Display for TextSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({},{}):({},{}):{}",
            self.from_line, self.from_col, self.to_line, self.to_col, self.lexeme
        )
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
        write!(f, "{}", self.kind)
    }
}
