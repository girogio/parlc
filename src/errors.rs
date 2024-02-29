pub enum ErrorKind {
    UnexpectedToken,
    UnexpectedEOF,
    InvalidNumber,
    InvalidAdditiveOp,
    InvalidMultiplicativeOp,
    InvalidRelationalOp,
    InvalidEqualityOp,
    InvalidAssignmentOp,
    InvalidFunctionCall,
    InvalidFunctionDecl,
    InvalidFormalParam,
    InvalidFormalParams,
    InvalidStatement,
    InvalidBlock,
    InvalidProgram,
    InvalidWhitespace,
    InvalidEOF,
}

pub struct SyntaxError {
    kind: ErrorKind,
    span: TextSpan,
}

impl SyntaxError {
    pub fn new(kind: ErrorKind, span: TextSpan) -> SyntaxError {
        SyntaxError { kind, span }
    }

    pub fn report(&self) {
        // pritns error nicey
        println!("Syntax Error: {:?}", self.kind);
        println!("Span: {:?}", self.span);
    }
}
