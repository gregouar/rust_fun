use engine::core::GameState;

use engine::text_rendering::{TextAlign, TextRenderer};

pub struct MainMenuState {}

impl MainMenuState {
    pub fn new() -> Box<Self> {
        Box::new(MainMenuState {})
    }
}

impl GameState for MainMenuState {
    fn entering(&self) {}
    fn revealing(&self) {}
    fn obscuring(&self) {}
    fn leaving(&self) {}

    fn update(&self) {}

    fn draw(&self, text_renderer: &dyn TextRenderer) {
        text_renderer.render_text("Welcome to the dungeon", TextAlign::Center);
        text_renderer.render_text("Main menu", TextAlign::Center);
        text_renderer.render_horizontal_separator();
        // TODO: Some kind of UI ?
        text_renderer.render_text("Please choose your option:", TextAlign::Left);
        text_renderer.render_text("  1) New game", TextAlign::Left);
        text_renderer.render_text("  2) Quit", TextAlign::Left);
    }
}
