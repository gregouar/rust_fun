use engine::core::{GameState, StateChangeAction};
use engine::input_handling::InputRecorder;
use engine::text_rendering::{TextAlign, TextRenderer};
use engine::ui::{TextUi, TextUiOrientation};
use std::time::Duration;

#[derive(Copy, Clone)]
enum SettingsMenuOptions {
    WindowsWidth,
    Back,
}

pub struct SettingsMenuState {
    ui: TextUi<SettingsMenuOptions>,
}

impl SettingsMenuState {
    pub fn new() -> Box<Self> {
        let mut ui = TextUi::<SettingsMenuOptions>::new(TextUiOrientation::Vertical);
        ui.add_option(
            String::from("Windows width"),
            '1',
            SettingsMenuOptions::WindowsWidth,
        );
        ui.add_option(String::from("Back"), 'B', SettingsMenuOptions::Back);
        Box::new(SettingsMenuState { ui })
    }
}

impl GameState for SettingsMenuState {
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
                SettingsMenuOptions::WindowsWidth => {}
                SettingsMenuOptions::Back => state_change_actions.push(StateChangeAction::PopState),
            };
        }

        state_change_actions
    }

    fn draw(&self, text_renderer: &dyn TextRenderer) {
        text_renderer.render_text("Settings", TextAlign::Center);
        text_renderer.render_horizontal_separator();
        text_renderer.render_text("Please choose your option:", TextAlign::Left);
        text_renderer.render_text_ui(&self.ui.renderable_text_ui());
    }
}
