use crate::input_handling::InputRecorder;

use super::{renderable_text_ui::RenderableUiOption, RenderableTextUi};

pub struct UiOption<T: Copy> {
    pub label: String,
    pub shortcut: char,
    pub value: T,
}

pub struct TextUi<T: Copy> {
    options: Vec<UiOption<T>>,
    chosen_option_value: Option<T>,
}

impl<T: Copy> TextUi<T> {
    pub fn new() -> Self {
        TextUi {
            options: Vec::new(),
            chosen_option_value: None,
        }
    }

    pub fn add_option(&mut self, label: String, shortcut: char, value: T) {
        self.options.push(UiOption {
            label,
            shortcut,
            value,
        })
    }

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn options_iter(&self) -> std::slice::Iter<UiOption<T>> {
        self.options.iter()
    }

    pub fn handle_input(&mut self, input_recorder: &InputRecorder) {
        self.chosen_option_value = None;
        for option in self.options.iter() {
            if input_recorder.key_released(option.shortcut as usize) {
                self.chosen_option_value = Some(option.value);
            }
        }
    }

    pub fn chosen_option(&self) -> Option<T> {
        self.chosen_option_value
    }

    pub fn renderable_text_ui(&self) -> RenderableTextUi {
        // let mut options = Vec::with_capacity(self.options.len());
        RenderableTextUi::new(
            self.options_iter()
                .map(|x| RenderableUiOption {
                    label: &x.label[..],
                    shortcut: x.shortcut,
                })
                .collect(),
        )
    }
}
