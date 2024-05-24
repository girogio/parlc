use crate::core::TokenKind;

use super::dfsa::{Category, DfsaBuilder};

#[derive(Debug)]
pub struct Transition<'a> {
    dfsa_builder: &'a mut DfsaBuilder,
    current_state: i32,
    transitions: Vec<((Category, i32), i32)>,
    final_state_token: Vec<(i32, TokenKind)>,
}

impl<'a> Transition<'a> {
    pub fn new(dfsa_builder: &'a mut DfsaBuilder) -> Self {
        Transition {
            dfsa_builder,
            current_state: 0,
            transitions: vec![],
            final_state_token: vec![],
        }
    }

    pub fn to<T: IntoIterator<Item = Category>>(&mut self, category: T) -> &mut Self {
        for cat in category {
            self.transitions.push((
                (
                    cat,
                    match self.current_state {
                        0 => 0,
                        _ => self.current_state + self.dfsa_builder.max_state - 1,
                    },
                ),
                self.dfsa_builder.max_state + self.current_state,
            ));
        }
        self.current_state += 1;
        self
    }

    pub fn repeated(&mut self) -> &mut Self {
        if let Some(((_, s1), s2)) = self.transitions.last().cloned() {
            let len = self.transitions.len();
            for i in (0..len).rev() {
                if self.transitions[i].0 .1 == s1 {
                    let mut new_tx = self.transitions[i];
                    new_tx.0 .1 = s2;
                    self.transitions.push(new_tx);
                } else {
                    break;
                }
            }
        }
        self
    }

    pub fn goes_to(&mut self, token: TokenKind) -> &mut Self {
        self.final_state_token
            .push((self.current_state + self.dfsa_builder.max_state - 1, token));
        self
    }

    pub fn done(&mut self) -> &mut DfsaBuilder {
        for ((cat, state), next_state) in &self.transitions {
            self.dfsa_builder.transition_table.insert(
                (
                    match state {
                        0 => 0,
                        _ => *state,
                    },
                    *cat,
                ),
                *next_state,
            );
        }

        for (state, token) in &self.final_state_token {
            self.dfsa_builder.state_to_token.insert(*state, *token);
            self.dfsa_builder.accepted_states.push(*state);
        }

        self.dfsa_builder.max_state += self.current_state - 1;

        self.dfsa_builder
    }
}
