use super::{InputHandler, InputRecorder, KeyAction};

use std::io;

pub struct ConsoleInputHandler {
    first_gather: bool,
}

impl ConsoleInputHandler {
    pub fn new() -> Box<Self> {
        Box::new(ConsoleInputHandler { first_gather: true })
    }
}

impl InputHandler for ConsoleInputHandler {
    fn gather_input(&mut self, input_recorder: &mut InputRecorder) {
        if self.first_gather {
            self.first_gather = false;
            return;
        }

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap(); // TODO: turn to ?

        input_recorder.record_text(&buffer);

        if buffer.chars().count() == 3 {
            let key_char = buffer
                .chars()
                .next()
                .unwrap()
                .to_uppercase()
                .next()
                .unwrap();
            input_recorder.record_key(key_char as usize, KeyAction::Released)
        }
    }
}
