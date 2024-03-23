use crate::core::Token;


pub type Ast = Box<AstNode>;

#[derive(Debug)]
pub enum AstNode {
    Program {
        statements: Vec<AstNode>,
    },
    VarDec {
        identifier: Token,
        r#type: Token,
        expression: Ast,
    },
    Block {
        statements: Vec<AstNode>,
    },
    Expression {
        casted_type: Option<Token>,
        expr: Ast,
    },
    SubExpression {
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
        x: Ast,
        y: Ast,
    },
    IntLiteral(Token),
    FloatLiteral(Token),
    BoolLiteral(Token),
    ColourLiteral(Token),
    FunctionCall {
        identifier: Token,
        args: Vec<Ast>,
    },
    ActualParams {
        params: Vec<Ast>,
    },
    Delay {
        expression: Ast,
    },
    Return {
        expression: Ast,
    },
    PadWriteBox {
        loc_x: Ast,
        loc_y: Ast,
        width: Ast,
        height: Ast,
        colour: Ast,
    },
    PadWrite {
        loc_x: Ast,
        loc_y: Ast,
        colour: Ast,
    },
    Identifier {
        token: Token,
    },
    If {
        condition: Ast,
        if_true: Ast,
        if_false: Option<Ast>,
    },
    For {
        initializer: Option<Ast>,
        condition: Ast,
        increment: Option<Ast>,
        body: Ast,
    },
    While {
        condition: Ast,
        body: Ast,
    },
    FormalParam {
        identifier: Token,
        param_type: Token,
    },
    FunctionDecl {
        identifier: Token,
        params: Vec<AstNode>,
        return_type: Token,
        block: Ast,
    },
    Print {
        expression: Ast,
    },
    Assignment {
        identifier: Token,
        expression: Ast,
    },
    EndOfFile,
    PadClear {
        expr: Ast,
    },
}

pub trait Visitor<T> {
    fn visit(&mut self, node: &AstNode) -> T;
}
