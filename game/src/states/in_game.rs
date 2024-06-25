use engine::core::GameState;

use engine::text_rendering::{TextAlign, TextRenderer};

pub struct InGameState {}

impl InGameState {
    pub fn new() -> Box<Self> {
        Box::new(InGameState {})
    }
}

impl GameState for InGameState {
    fn entering(&self) {}
    fn revealing(&self) {}
    fn obscuring(&self) {}
    fn leaving(&self) {}

    fn update(&self) {}

    fn draw(&self, text_renderer: &dyn TextRenderer) {
        text_renderer.render_text("Dungeon Entrance", TextAlign::Center);
        text_renderer.render_text(
            "After walking for 7 days and 7 nights, you finally arrive at the dungeon entrance.",
            TextAlign::Left,
        );
        text_renderer.render_text("A huge oak door bar your way...", TextAlign::Left);
        text_renderer.render_horizontal_separator();
        // TODO: Some kind of UI ?
        text_renderer.render_text("Please choose your option:", TextAlign::Left);
        text_renderer.render_text("  1) Do something", TextAlign::Left);
        text_renderer.render_text("  Q) Quit", TextAlign::Left);
    }
}
