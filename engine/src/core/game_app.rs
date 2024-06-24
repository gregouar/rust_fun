use std::error::Error;

use super::{GameState, GameStatesManager};
use crate::text_rendering::TextRenderer;
pub struct GameApp {
    text_renderer: Box<dyn TextRenderer>,
    states_manager: GameStatesManager,
    is_running: bool,
}

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
            self.states_manager.update();
            self.states_manager.draw(self.text_renderer);
        }

        // self.text_renderer.render_text("Welcome to the dungeon!");

        Ok(())
    }

    fn initialize(&mut self, starting_state: Box<dyn GameState>) -> Result<(), Box<dyn Error>> {
        self.states_manager.switch_state(starting_state);

        Ok(())
    }
}
