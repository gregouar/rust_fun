use super::{TextAlign, TextRenderer};
use crate::ui::RenderableTextUi;

pub struct ConsoleTextRenderer {
    screen_width: usize,
    clear_screen_line: String,
    horizontal_separator: String,
}

impl ConsoleTextRenderer {
    pub fn new(screen_width: usize) -> Box<ConsoleTextRenderer> {
        Box::new(ConsoleTextRenderer {
            screen_width,
            clear_screen_line: "=".repeat(screen_width),
            horizontal_separator: "-".repeat(screen_width),
        })
    }
}

impl TextRenderer for ConsoleTextRenderer {
    fn clear(&self) {
        println!("{}", self.clear_screen_line);
    }

    fn render_text(&self, text: &str, text_align: TextAlign) {
        //TODO: Line breaking ?
        let str_to_print = match text_align {
            TextAlign::Left => format!("{: <1$}", text, self.screen_width),
            TextAlign::Center => format!("{: ^1$}", text, self.screen_width),
            TextAlign::Right => format!("{: >1$}", text, self.screen_width),
        };

        println!("{}", str_to_print)
    }

    fn render_horizontal_separator(&self) {
        println!("{}", self.horizontal_separator);
    }

    fn render_text_ui(&self, text_ui: &RenderableTextUi) {
        for option in text_ui.options_iter() {
            let formatted_label = format!(" {}) {}", option.shortcut, option.label);
            self.render_text(&formatted_label, TextAlign::Left);
        }
    }
}
