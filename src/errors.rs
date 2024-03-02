use crate::core::tokens::TextSpan;

#[derive(Debug)]
pub enum ErrorKind {
    Lexical(LexicalErrorKind),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Option<TextSpan>,
}

#[derive(Debug)]
pub struct LexicalError {
    kind: LexicalErrorKind,
    span: TextSpan,
}

#[derive(Debug)]
pub enum LexicalErrorKind {
    InvalidCharacter,
    UnterminatedString,
    UnterminatedComment,
}

impl LexicalError {
    pub fn new(kind: LexicalErrorKind, span: TextSpan) -> LexicalError {
        LexicalError { kind, span }
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
