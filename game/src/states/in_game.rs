use engine::core::{GameState, StateChangeAction};

use super::{MainMenuState, SettingsMenuState};
use engine::input_handling::InputRecorder;
use engine::text_rendering::{TextAlign, TextRenderer};
use engine::ui::{TextUi, TextUiOrientation};
use std::time::Duration;

#[derive(Copy, Clone)]
enum InGameMenuOptions {
    Inventory,
    CharacterSheet,
    Settings,
    Quit,
}

pub struct InGameState {
    obscured: bool,
    game_ui: TextUi<u8>,
    menu_ui: TextUi<InGameMenuOptions>,
}

impl InGameState {
    pub fn new() -> Box<Self> {
        let mut menu_ui = TextUi::<InGameMenuOptions>::new(TextUiOrientation::Horizontal);
        menu_ui.add_option(String::from("Inventory"), 'I', InGameMenuOptions::Inventory);
        menu_ui.add_option(
            String::from("Character sheet"),
            'C',
            InGameMenuOptions::CharacterSheet,
        );
        menu_ui.add_option(String::from("Settings"), 'S', InGameMenuOptions::Settings);
        menu_ui.add_option(String::from("Quit"), 'Q', InGameMenuOptions::Quit);

        let mut game_ui = TextUi::new(TextUiOrientation::Vertical);
        // TODO: move somewhere else, should be dynamic
        game_ui.add_option(String::from("Do something"), '1', 1);
        game_ui.add_option(String::from("Do something else"), '2', 2);
        Box::new(InGameState {
            obscured: false,
            menu_ui,
            game_ui,
        })
    }
}

impl GameState for InGameState {
    fn entering(&mut self) {}
    fn revealing(&mut self) {
        self.obscured = false;
    }
    fn obscuring(&mut self) {
        self.obscured = true;
    }
    fn leaving(&mut self) {}

    fn handle_input(&mut self, input_recorder: &InputRecorder) {
        if self.obscured {
            return;
        }

        self.game_ui.handle_input(input_recorder);
        self.menu_ui.handle_input(input_recorder);
    }

    fn update(&mut self, _elapsed_time: Duration) -> Vec<StateChangeAction> {
        let mut state_change_actions = Vec::new();

        if let Some(option) = self.game_ui.chosen_option() {
            match option {
                _ => {}
            };
        }

        if let Some(option) = self.menu_ui.chosen_option() {
            match option {
                InGameMenuOptions::Inventory => {}
                InGameMenuOptions::CharacterSheet => {}
                InGameMenuOptions::Settings => state_change_actions
                    .push(StateChangeAction::PushState(SettingsMenuState::new())),
                InGameMenuOptions::Quit => {
                    state_change_actions.push(StateChangeAction::SwitchState(MainMenuState::new()))
                }
            };
        }

        state_change_actions
    }

    fn draw(&self, text_renderer: &dyn TextRenderer) {
        if self.obscured {
            return;
        }

        text_renderer.render_text("Dungeon Entrance", TextAlign::Center);
        text_renderer.render_text(
            "After walking for 7 days and 7 nights, you finally arrive at the dungeon entrance.",
            TextAlign::Left,
        );
        text_renderer.render_text("A huge oak door bar your way...", TextAlign::Left);
        text_renderer.render_horizontal_separator();
        text_renderer.render_text_ui(&self.menu_ui.renderable_text_ui());
        text_renderer.render_horizontal_separator();
        text_renderer.render_text("What do you want to do ?", TextAlign::Left);
        text_renderer.render_text_ui(&self.game_ui.renderable_text_ui());
    }
}
