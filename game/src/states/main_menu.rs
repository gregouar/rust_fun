use super::InGameState;
use engine::core::{GameState, StateChangeAction};
use engine::input_handling::InputRecorder;
use engine::text_rendering::{TextAlign, TextRenderer};
use engine::ui::TextUi;
use std::time::Duration;

#[derive(Copy, Clone)]
enum MainMenuOptions {
    NewGame,
    Options,
    Quit,
}

pub struct MainMenuState {
    ui: TextUi<MainMenuOptions>,
}

impl MainMenuState {
    pub fn new() -> Box<Self> {
        let mut ui = TextUi::<MainMenuOptions>::new();
        ui.add_option(String::from("New Game"), '1', MainMenuOptions::NewGame);
        ui.add_option(String::from("Options"), '2', MainMenuOptions::Options);
        ui.add_option(String::from("Quit"), 'Q', MainMenuOptions::Quit);
        Box::new(MainMenuState { ui })
    }
}

impl GameState for MainMenuState {
    fn entering(&mut self) {}
    fn revealing(&mut self) {}
    fn obscuring(&mut self) {}
    fn leaving(&mut self) {}

    fn handle_input(&mut self, input_recorder: &InputRecorder) {
        self.ui.handle_input(input_recorder);
    }

    fn update(&mut self, _elapsed_time: Duration) -> Vec<StateChangeAction> {
        let mut state_change_actions = Vec::new();
        if let Some(option) = self.ui.chosen_option() {
            match option {
                MainMenuOptions::NewGame => {
                    state_change_actions.push(StateChangeAction::SwitchState(InGameState::new()))
                }
                MainMenuOptions::Options => {}
                MainMenuOptions::Quit => state_change_actions.push(StateChangeAction::Stop),
            };
        }
        state_change_actions
    }

    fn draw(&self, text_renderer: &dyn TextRenderer) {
        text_renderer.render_text("Welcome to the dungeon", TextAlign::Center);
        text_renderer.render_text("Main menu", TextAlign::Center);
        text_renderer.render_horizontal_separator();
        text_renderer.render_text("Please choose your option:", TextAlign::Left);
        text_renderer.render_text_ui(&self.ui.renderable_text_ui());
    }
}
