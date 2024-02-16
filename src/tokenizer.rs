pub enum TokenKind {
    Number,
    Operator,
    Parenthesis,
    Function,
    Variable,
    Comma,
    Assignment,
    EndOfInput,
}

pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

pub struct Tokenizer;

impl Tokenizer {
    pub fn new() -> Self {
        Tokenizer
    }

    pub fn tokenize(&self, input: &str) -> Vec<String> {
        input.split_whitespace().map(|s| s.to_string()).collect()
    }
}
