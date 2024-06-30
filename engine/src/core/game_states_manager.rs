use super::{GameState, StateChangeAction};

use crate::{input_handling::InputRecorder, text_rendering::TextRenderer};
use std::time::Duration;

pub struct GameStatesManager {
    states_stack: Vec<Box<dyn GameState>>,
    next_state: Option<Box<dyn GameState>>,
    stop_request: bool,
}

impl GameStatesManager {
    pub fn new() -> Self {
        GameStatesManager {
            states_stack: Vec::new(),
            next_state: None,
            stop_request: false,
        }
    }

    pub fn peek_state(&self) -> Option<&Box<dyn GameState>> {
        self.states_stack.last()
    }

    pub fn stop(&mut self) {
        self.stop_request = true;
    }

    pub fn switch_state(&mut self, new_state: Box<dyn GameState>) {
        self.next_state = Some(new_state);
    }

    pub fn push_state(&mut self, game_state: Box<dyn GameState>) {
        if let Some(top_state) = self.states_stack.last_mut() {
            top_state.obscuring();
        }

        self.states_stack.push(game_state);
        self.states_stack.last_mut().unwrap().entering();
    }

    pub fn pop_state(&mut self) {
        let mut game_state = self
            .states_stack
            .pop()
            .expect("Attempted to pop from an empty game state stack");
        game_state.leaving();

        if let Some(top_state) = self.states_stack.last_mut() {
            top_state.revealing();
        }
    }

    pub fn handle_input(&mut self, input_recorder: &InputRecorder) {
        for state in self.states_stack.iter_mut() {
            state.handle_input(input_recorder);
        }
    }

    pub fn update_states(&mut self, elapsed_time: Duration) {
        let mut state_actions: Vec<StateChangeAction> = Vec::new();
        for state in self.states_stack.iter_mut() {
            state_actions.extend(state.update(elapsed_time));
        }
        for state_action in state_actions {
            match state_action {
                StateChangeAction::SwitchState(new_state) => self.switch_state(new_state),
                StateChangeAction::PushState(new_state) => self.push_state(new_state),
                StateChangeAction::PopState => self.pop_state(),
                StateChangeAction::Stop => self.stop(),
            }
        }
        self._handle_switch_state();
    }

    pub fn draw_states(&self, text_renderer: &dyn TextRenderer) {
        for state in &self.states_stack {
            state.draw(text_renderer);
        }
    }

    fn _handle_switch_state(&mut self) {
        if self.stop_request {
            self._clear_states_stack();
            self.stop_request = false;
            return;
        }

        if let Some(new_state) = self.next_state.take() {
            self._clear_states_stack();
            self.push_state(new_state);
        }
    }

    fn _clear_states_stack(&mut self) {
        for state in self.states_stack.iter_mut() {
            state.leaving()
        }
        self.states_stack.clear();
    }
}
