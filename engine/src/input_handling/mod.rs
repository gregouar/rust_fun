pub mod console_input_handler;
mod input_handler_trait;
mod input_recorder;

pub use input_handler_trait::InputHandler;
pub use input_recorder::{InputRecorder, KeyAction};
