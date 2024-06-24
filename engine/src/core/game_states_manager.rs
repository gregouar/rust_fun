use super::GameState;

pub struct GameStatesManager {
    states_stack: Vec<Box<dyn GameState>>,
}

impl GameStatesManager {
    pub fn new() -> Self {
        GameStatesManager {
            states_stack: Vec::new(),
        }
    }

    pub fn switch_state(&mut self, new_state: Box<dyn GameState>) {
        self.states_stack.push(new_state)
    }
}
