use super::GameState;

use crate::text_rendering::TextRenderer;

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
        self.states_stack.push(new_state) // TODO: Replace by delayed switch
    }

    pub fn update_states(&self) {
        for state in &self.states_stack {
            state.update();
        }
    }

    pub fn draw_states(&self, text_renderer: &dyn TextRenderer) {
        for state in &self.states_stack {
            state.draw(text_renderer);
        }
    }
}
