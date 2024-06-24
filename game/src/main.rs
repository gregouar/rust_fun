use engine::text_rendering::console_text_renderer::ConsoleTextRenderer;

use engine::core::GameApp;
use game::states::MainMenuState;
use std::error::Error;
use std::process;

fn main() {
    let text_renderer = ConsoleTextRenderer::new();
    let mut app = GameApp::new(text_renderer);

    if let Err(e) = app.run(MainMenuState::new()) {
        handle_exit_error(e);
        process::exit(1);
    }
}

fn handle_exit_error(error: Box<dyn Error>) {
    eprintln!("Application error: {error}");
}
