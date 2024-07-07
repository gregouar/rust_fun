use std::error::Error;

use super::{GameState, GameStatesManager};
use crate::{
    input_handling::{InputHandler, InputRecorder},
    text_rendering::TextRenderer,
};
use std::time::Instant;
pub struct GameApp {
    text_renderer: Box<dyn TextRenderer>,
    input_handler: Box<dyn InputHandler>,
    input_recorder: InputRecorder,
    states_manager: GameStatesManager,
}

impl GameApp {
    pub fn new(text_renderer: Box<dyn TextRenderer>, input_handler: Box<dyn InputHandler>) -> Self {
        GameApp {
            text_renderer,
            input_handler,
            input_recorder: InputRecorder::new(),
            states_manager: GameStatesManager::new(),
        }
    }

    pub fn run(&mut self, starting_state: Box<dyn GameState>) -> Result<(), Box<dyn Error>> {
        self.initialize(starting_state)?;

        let mut start = Instant::now();

        loop {
            let elapsed_time = start.elapsed();
            start = Instant::now();

            self.input_recorder.update();
            self.input_handler.gather_input(&mut self.input_recorder);

            self.states_manager.handle_input(&self.input_recorder);
            self.states_manager.update_states(elapsed_time);

            if self.states_manager.peek_state().is_none() {
                break;
            }

            self.text_renderer.clear()?;
            self.states_manager.draw_states(&(*self.text_renderer));
        }

        Ok(())
    }

    fn initialize(&mut self, starting_state: Box<dyn GameState>) -> Result<(), Box<dyn Error>> {
        self.states_manager.switch_state(starting_state);

        Ok(())
    }
}
