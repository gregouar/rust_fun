const MAX_KEY_VALUE: usize = 256;

pub enum KeyAction {
    Pressed,
    Repeated,
    Released,
}

pub struct InputRecorder {
    text_entered: String,
    key_released: [bool; MAX_KEY_VALUE],
}

impl InputRecorder {
    pub fn new() -> Self {
        InputRecorder {
            text_entered: String::new(),
            key_released: [false; MAX_KEY_VALUE],
        }
    }

    pub fn get_text_entered(&self) -> &str {
        &self.text_entered[..]
    }

    pub fn key_released(&self, key: usize) -> bool {
        if key >= MAX_KEY_VALUE {
            return false;
        }
        self.key_released[key]
    }

    pub fn record_key(&mut self, key: usize, action: KeyAction) {
        if key >= MAX_KEY_VALUE {
            return;
        }
        match action {
            KeyAction::Pressed => {}
            KeyAction::Repeated => {}
            KeyAction::Released => {
                self.key_released[key] = true;
            }
        }
    }

    pub fn record_text(&mut self, text: &str) {
        self.text_entered.push_str(text);
    }

    pub fn update(&mut self) {
        self.text_entered = String::new();

        for i in 0..MAX_KEY_VALUE {
            self.key_released[i] = false
        }
    }
}
