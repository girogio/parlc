use std::fmt::Display;

use crate::scanner::token::TokenKind;

// #[derive(Debug, PartialEq, Eq, Default)]
// pub enum Category {
//     #[default]
//     Other,
//     Digit,
//     Letter,
//     Identifier,
//     LParenthesis,
//     RParentesis,
//     LBrace,
//     RBrace,
//     LBracket,
//     RBracket,
//     Comma,
//     Semicolon,
//     Colon,
//     Period,
// }

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub enum StateType {
    Start,
    Accepting,
    Rejecting,
}

#[derive(Debug, PartialEq, Clone)]
pub struct State {
    pub id: usize,
    pub state_type: StateType,
    pub token_kind: TokenKind,
}

impl State {
    pub fn new(id: usize, state_type: StateType, token_kind: TokenKind) -> Self {
        Self {
            id,
            state_type,
            token_kind,
        }
    }

    pub fn is_accepting(&self) -> bool {
        self.state_type == StateType::Accepting
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            id: 0,
            state_type: StateType::Rejecting,
            token_kind: TokenKind::Invalid,
        }
    }
}
