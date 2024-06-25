use std::error::Error;

use super::{GameState, GameStatesManager};
use crate::text_rendering::TextRenderer;
pub struct GameApp {
    text_renderer: Box<dyn TextRenderer>,
    states_manager: GameStatesManager,
    is_running: bool,
}

// Use generic type for text_renderer ?
impl GameApp {
    pub fn new(text_renderer: Box<dyn TextRenderer>) -> Self {
        GameApp {
            text_renderer,
            states_manager: GameStatesManager::new(),
            is_running: true,
        }
    }

    pub fn run(&mut self, starting_state: Box<dyn GameState>) -> Result<(), Box<dyn Error>> {
        self.initialize(starting_state)?;

        while self.is_running {
            self.states_manager.update_states();

            self.text_renderer.clear();
            self.states_manager.draw_states(&(*self.text_renderer));
        }

        Ok(())
    }

    fn initialize(&mut self, starting_state: Box<dyn GameState>) -> Result<(), Box<dyn Error>> {
        self.states_manager.switch_state(starting_state);

        Ok(())
    }
}
