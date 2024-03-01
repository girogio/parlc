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

enum ErrorKind {
    Lexical(LexicalErrorKind),
}

pub struct Error {
    kind: ErrorKind,
    span: Option<TextSpan>,
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
        println!("Error: {:?}", self.kind);
        println!("Span: {:?}", self.span);
    }
}

impl Error {
    pub fn report(&self) {
        match &self.kind {
            ErrorKind::Lexical(kind) => {
                println!("Error: {:?}", kind);
                println!("Span: {:?}", self.span);
            }
        }
    }
}

impl From<LexicalError> for Error {
    fn from(error: LexicalError) -> Error {
        Error {
            kind: ErrorKind::Lexical(error.kind),
            span: Some(error.span),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
