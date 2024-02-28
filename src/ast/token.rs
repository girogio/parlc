#[derive(Debug)]
#[allow(dead_code)]
pub enum TokenKind {
    Letter,
    Digit,
    Hex,
    Type,
    BooleanLiteral,
    IntegerLiteral(i64),
    FloatLiteral,
    ColourLiteral,
    PadWidth,
    PadHeight,
    PadRead,
    PadRandI,
    Literal,
    Identifier,
    MultOp,
    AddOp,
    RelationalOp,
    ActualParams,
    FunctionCall,
    SubExpr,
    Unary,
    Factor,
    Term,
    SimpleExpr,
    Expr,
    Assignment,
    PrintStatement,
    DelayStatement,
    PixelStatement,
    RtrnStatement,
    IfStatement,
    ForStatement,
    WhileStatement,
    FormalParam,
    FormalParams,
    FunctionDecl,
    Statement,
    Block,
    Program,
    // Utility tokens
    Whitespace,
    BadToken,
    EOF,
}

#[derive(Debug)]
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

    pub fn length(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Token {
        Token { kind, span }
    }
}
