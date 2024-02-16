pub struct Lexer;

impl Lexer {
    pub fn new() -> Self {
        Lexer
    }

    pub fn lex(&self, input: &str) -> Vec<String> {
        input.split_whitespace().map(|s| s.to_string()).collect()
    }
}
