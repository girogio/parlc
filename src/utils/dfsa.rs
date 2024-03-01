use std::marker::PhantomData;

pub struct Dfsa<State, Category: Sized, Delta>
where
    State: PartialEq + Eq,
    Delta: Fn(State, Category) -> State,
{
    accepted_states: Vec<State>,
    category: PhantomData<Category>,
    start_state: State,
    delta: Delta,
}

impl<State: PartialEq + Eq + Copy, Category, Delta: Fn(State, Category) -> State>
    Dfsa<State, Category, Delta>
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

    pub fn delta(&self, state: State, category: Category) -> State {
        (self.delta)(state, category)
    }
}
