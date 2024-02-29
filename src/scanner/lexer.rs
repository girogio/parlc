use crate::scanner::token::{Token, TokenKind};

use super::token::TextSpan;

#[derive(Debug, PartialEq, Eq)]
pub enum Category {
    Other,
    Whitespace,
    Register,
    Digit,
}

/// Why is the lifetime parameter 'a necessary?
///
/// The lifetime parameter 'a is necessary because the Lexer struct contains a
/// reference to the input string. The lifetime parameter 'a ensures that the
/// input string outlives the Lexer instance.
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
        let state_0 = 0;
        let mut state_stack = vec![state_0];
        state_stack.push(-1);

        let mut lexeme = String::new();
        let mut current_state = state_0;

        println!("lexing: {}", self.input);

        println!("lexeme: {}, in state: {}", lexeme, current_state);

        while current_state != -1 {
            if self.accepted_states.contains(&current_state) {
                state_stack.clear();
            }
            state_stack.push(current_state);

            match self.get_current_char() {
                Some(c) => {
                    lexeme += &c.to_string();
                    self.current_pos += 1;

                    let cat = Category::from(c);
                    current_state = (self.transition_table)(current_state, &cat);
                }
                None => break,
            };
        }

        while !self.accepted_states.contains(&current_state) && !current_state == -1 {
            if state_stack.last() == Some(&-2) {
                current_state = state_stack.pop().unwrap();
                // truncaet lexeme to the last accepted state
                lexeme = lexeme.chars().take(self.current_pos - 1).collect();
            }
        }

        match self.accepted_states.contains(&current_state) {
            true => {
                let end = self.current_pos;
                let span = TextSpan::new(end - lexeme.len(), end, &lexeme);
                let token = Token::new(current_state.into(), span);
                Some(token)
            }
            false => None,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            println!("{:?}", token);
            tokens.push(token);
        }

        tokens
    }
}

pub fn delta(state: i32, category: &Category) -> i32 {
    match (state, category) {
        (0, Category::Register) => 1,
        (1, Category::Digit) => 2,
        (2, Category::Digit) => 2,
        (0, Category::Whitespace) => 3,
        (2, Category::Whitespace) => 3,
        _ => -1,
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
