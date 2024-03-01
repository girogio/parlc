use crate::{
    scanner::token::{Token, TokenKind},
    utils::Stream,
};

use super::token::TextSpan;

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    Other,
    Whitespace,
    Register,
    Digit,
}

const ERR_STATE: i32 = -2;
const BAD_STATE: i32 = -1;
const START_STATE: i32 = 0;

pub struct Lexer<B: Stream> {
    buffer: B,
    accepted_states: Vec<i32>,
}

impl<B: Stream> Lexer<B> {
    pub fn new(input: &str) -> Self {
        Lexer {
            buffer: B::new(input),
            accepted_states: vec![2, 3],
        }
    }

    fn is_accepted(&self, state: &i32) -> bool {
        self.accepted_states.contains(state)
    }

    fn next_token(&mut self) -> Token {
        let mut state = START_STATE;
        let mut lexeme = String::new();
        let mut stack = vec![BAD_STATE];
        let start = self.buffer.get_input_pointer();

        while state != ERR_STATE {
            let c = self.buffer.next_char();
            lexeme += &c.to_string();

            if self.is_accepted(&state) {
                stack = vec![BAD_STATE];
            }

            // Save the current state
            stack.push(state);

            // Perform the transition
            let cat = Category::from(c);
            state = self.delta(state, &cat);
        }

        while !self.is_accepted(&state) && state != BAD_STATE {
            state = stack.pop().unwrap();
            if state != BAD_STATE {
                lexeme.pop();
                self.buffer.rollback();
            }
        }

        let text_span = TextSpan::new(start, self.buffer.get_input_pointer(), &lexeme);

        Token::new(
            match self.is_accepted(&state) {
                true => TokenKind::from(state),
                false => TokenKind::Invalid,
            },
            text_span,
        )
    }

    fn delta(&self, state: i32, category: &Category) -> i32 {
        match (state, category) {
            (0, Category::Register) => 1,
            (0, Category::Whitespace) => 3,
            (1, Category::Digit) => 2,
            (2, Category::Digit) => 2,
            (3, Category::Whitespace) => 3,
            (3, _) => 0,
            _ => ERR_STATE,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();

            if token.kind == TokenKind::Invalid {
                break;
            }

            tokens.push(token);
        }

        tokens
    }
}

impl From<char> for Category {
    fn from(c: char) -> Self {
        match c {
            'r' => Category::Register,
            '0'..='9' => Category::Digit,
            ' ' | '\n' | '\t' => Category::Whitespace,
            _ => Category::Other,
        }
    }
}

impl From<i32> for TokenKind {
    fn from(i: i32) -> Self {
        match i {
            2 => TokenKind::Register,
            3 => TokenKind::Whitespace,
            _ => TokenKind::Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::SimpleBuffer;

    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_lex() {
        let input = "r0123 r2456 \n r1234";
        let accepted_states = vec![2, 3];
        let mut lexer: Lexer<SimpleBuffer> = Lexer::new(input);
        let tokens = lexer.lex();

        for token in tokens {
            println!("{:?}", token)
        }
    }
}
