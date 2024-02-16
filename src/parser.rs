use crate::tokenizer::Token;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
