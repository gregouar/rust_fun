use super::input_recorder::InputRecorder;

pub trait InputHandler {
    fn gather_input(&mut self, input_recorder: &mut InputRecorder);
}
