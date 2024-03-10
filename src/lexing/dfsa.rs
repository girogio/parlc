use std::marker::PhantomData;

pub struct Dfsa<State, Alpha>
where
    State: PartialEq + Eq,
{
    accepted_states: Vec<State>,
    alpha: PhantomData<Alpha>,
    start_state: State,
    delta: fn(State, Alpha) -> State,
}

impl<State: PartialEq + Eq + Copy, Alpha> Dfsa<State, Alpha> {
    pub fn new(
        accepted_states: Vec<State>,
        start_state: State,
        delta: fn(State, Alpha) -> State,
    ) -> Self {
        Dfsa {
            accepted_states,
            alpha: PhantomData,
            start_state,
            delta,
        }
    }

    pub fn start_state(&self) -> &State {
        &self.start_state
    }

    pub fn is_accepting(&self, state: &State) -> bool {
        self.accepted_states.contains(state)
    }

    pub fn delta(&self, state: State, category: Alpha) -> State {
        (self.delta)(state, category)
    }
}
