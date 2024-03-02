use std::marker::PhantomData;

pub struct Dfsa<State, Alpha, Delta>
where
    State: PartialEq + Eq,
    Delta: Fn(State, Alpha) -> State,
{
    accepted_states: Vec<State>,
    category: PhantomData<Alpha>,
    start_state: State,
    delta: Delta,
}

impl<State: PartialEq + Eq + Copy, Alpha, Delta: Fn(State, Alpha) -> State>
    Dfsa<State, Alpha, Delta>
{
    pub fn new(accepted_states: Vec<State>, start_state: State, delta: Delta) -> Self {
        Dfsa {
            accepted_states,
            category: PhantomData,
            start_state,
            delta,
        }
    }

    pub fn start_state(&self) -> State {
        self.start_state
    }

    pub fn is_accepting(&self, state: &State) -> bool {
        self.accepted_states.contains(state)
    }

    pub fn delta(&self, state: State, category: Alpha) -> State {
        (self.delta)(state, category)
    }
}
