#[derive(Debug, PartialEq, Clone)]
// pub enum TokenKind {
//     Invalid,
//     Whitespace,
//     Letter,
//     Digit,
//     Hex,
//     Type,
//     BooleanLiteral,
//     IntegerLiteral,
//     FloatLiteral,
//     ColourLiteral,
//     PadWidth,
//     PadHeight,
//     PadRead,
//     PadRandI,
//     Literal,
//     Identifier,
//     MultOp,
//     AddOp,
//     RelationalOp,
//     ActualParams,
//     FunctionCall,
//     SubExpr,
//     Unary,
//     Factor,
//     Term,
//     SimpleExpr,
//     Expr,
//     Assignment,
//     PrintStatement,
//     DelayStatement,
//     PixelStatement,
//     RtrnStatement,
//     IfStatement,
//     ForStatement,
//     WhileStatement,
//     FormalParam,
//     FormalParams,
//     FunctionDecl,
//     Statement,
//     Block,
//     Program,
// }
pub enum TokenKind {
    Whitespace,
    Invalid,
    Register,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TextSpan {
    start: usize,
    end: usize,
    pub lexeme: String,
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
