use crate::ui::RenderableTextUi;
use std::error::Error;

pub enum TextAlign {
    Left,
    Center,
    Right,
}

pub trait TextRenderer {
    fn clear(&self) -> Result<(), Box<dyn Error>>;
    fn render_text(&self, text: &str, text_align: TextAlign);
    fn render_horizontal_separator(&self);
    fn render_text_ui(&self, text_ui: &RenderableTextUi);
}
