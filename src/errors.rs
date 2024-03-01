use crate::scanner::token::TextSpan;

#[derive(Debug)]
pub enum LexicalErrorKind {
    InvalidCharacter,
    UnterminatedString,
    UnterminatedComment,
    UnterminatedBlockComment,
    InvalidEscape,
    InvalidNumber,
    InvalidIdentifier,
}

pub struct LexicalError {
    kind: LexicalErrorKind,
    span: TextSpan,
}

impl LexicalError {
    pub fn new(kind: LexicalErrorKind, span: TextSpan) -> LexicalError {
        LexicalError { kind, span }
    }

    pub fn report(&self) {
        // pritns error nicey
        println!("Error: {:?}", self.kind);
        println!("Span: {:?}", self.span);
    }
}
