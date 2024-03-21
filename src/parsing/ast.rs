use crate::core::Token;
use crate::utils::Result;

#[derive(Debug)]
pub enum AstNode {
    Program {
        statements: Vec<AstNode>,
    },
    VarDec {
        identifier: Ast,
        var_type: Option<Token>,
        expression: Ast,
    },
    Identifier {
        token: Token,
    },
    Block {
        statements: Vec<AstNode>,
    },
    Expression {
        casted_type: Option<Token>,
        bin_op: Ast,
    },
    UnaryOp {
        operator: Token,
        expr: Ast,
    },
    BinOp {
        left: Ast,
        operator: Token,
        right: Ast,
    },
    PadWidth,
    PadRandI {
        upper_bound: Ast,
    },
    PadHeight,
    PadRead {
        first: Ast,
        second: Ast,
    },
    IntLiteral(Token),
    FloatLiteral(Token),
    BoolLiteral(Token),
    ColourLiteral(Token),
    FunctionCall {
        identifier: Ast,
        args: Vec<Ast>,
    },
    ActualParams {
        params: Vec<Ast>,
    },
    Delay {
        expression: Box<AstNode>,
    },
    Return {
        expression: Box<AstNode>,
    },
    PadWriteBox {
        loc_x: Box<AstNode>,
        loc_y: Box<AstNode>,
        width: Box<AstNode>,
        height: Box<AstNode>,
        colour: Box<AstNode>,
    },
    PadWrite {
        loc_x: Box<AstNode>,
        loc_y: Box<AstNode>,
        colour: Box<AstNode>,
    },
    If {
        condition: Box<AstNode>,
        if_true: Box<AstNode>,
        if_false: Option<Box<AstNode>>,
    },
    For {
        initializer: Box<Option<AstNode>>,
        condition: Box<AstNode>,
        increment: Box<Option<AstNode>>,
        body: Box<AstNode>,
    },
    While {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },
    FormalParam {
        identifier: Box<AstNode>,
        param_type: Token,
    },
    FunctionDecl {
        identifier: Box<AstNode>,
        params: Vec<AstNode>,
        return_type: Token,
        block: Box<AstNode>,
    },
    Print {
        expression: Box<AstNode>,
    },
    Assignment {
        identifier: Box<AstNode>,
        expression: Box<AstNode>,
    },
    EndOfFile,
    PadClear {
        expr: Ast,
    },
}

pub type Ast = Box<AstNode>;

pub trait Visitor {
    fn visit(&mut self, node: &AstNode) -> Result<()>;
}
