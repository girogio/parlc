use crate::scanner::token::{Token, TokenKind};

use super::token::TextSpan;

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    Other,
    Whitespace,
    Register,
    Digit,
}

static ERR_STATE: i32 = -2;
static BAD_STATE: i32 = -1;

pub struct Lexer<'a, F: Fn(i32, &Category) -> i32> {
    input: &'a str,
    current_pos: usize,
    transition_table: F,
    accepted_states: Vec<i32>,
}

impl<'a, F: Fn(i32, &Category) -> i32> Lexer<'a, F> {
    pub fn new(input: &'a str, transition_table: F, accepted_states: &[i32]) -> Self {
        Lexer {
            input,
            current_pos: 0,
            transition_table,
            accepted_states: accepted_states.to_vec(),
        }
    }

    fn get_current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn next_token(&mut self) -> Option<Token> {
        let mut state = 0;
        let mut lexeme = String::new();
        let mut stack = vec![BAD_STATE];

        while state != ERR_STATE {
            if let Some(c) = self.get_current_char() {
                self.current_pos += 1;
                lexeme += &c.to_string();
                if self.is_accepted(&state) {
                    stack = vec![BAD_STATE];
                }
                stack.push(state);

                let cat = Category::from(c);
                state = (self.transition_table)(state, &cat);
            } else {
                // lexeme.pop();
                break;
            }
        }

        while !self.is_accepted(&state) && state != BAD_STATE {
            state = stack.pop().unwrap();
            if state != BAD_STATE {
                lexeme.pop();
                self.rollback();
            }
        }

        match self.is_accepted(&state) {
            true => {
                let token_kind = TokenKind::from(state);
                let text_span =
                    TextSpan::new(self.current_pos - lexeme.len(), self.current_pos, &&lexeme);
                let token = Token::new(token_kind, text_span);
                Some(token)
            }
            false => None,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }

    fn is_accepted(&self, state: &i32) -> bool {
        self.accepted_states.contains(state)
    }

    fn rollback(&mut self) {
        self.current_pos -= 1;
    }
}

pub fn delta(state: i32, category: &Category) -> i32 {
    match (state, category) {
        (0, Category::Register) => 1,
        (1, Category::Digit) => 2,
        (2, Category::Digit) => 2,
        (2, Category::Whitespace) => 1,
        (0, Category::Whitespace) => 3,
        (3, Category::Whitespace) => 3,
        _ => ERR_STATE,
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
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_lex() {
        let input = "r0123  r223 bruh r123";
        let accepted_states = vec![2, 3];
        let mut lexer = Lexer::new(input, delta, &accepted_states);
        let tokens = lexer.lex();

        for token in tokens {
            println!("{:?}", token)
        }
    }
}
