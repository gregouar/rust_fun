use crate::{input_handling::InputRecorder, text_rendering::TextRenderer};
use std::time::Duration;

pub enum StateChangeAction {
    SwitchState(Box<dyn GameState>),
    PushState(Box<dyn GameState>),
    PopState,
    Stop,
}

pub trait GameState {
    fn entering(&mut self);
    fn revealing(&mut self);
    fn obscuring(&mut self);
    fn leaving(&mut self);

    fn handle_input(&mut self, input_recorder: &InputRecorder);
    fn update(&mut self, elapsed_time: Duration) -> Vec<StateChangeAction>;
    fn draw(&self, text_renderer: &dyn TextRenderer);
}
