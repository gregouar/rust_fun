use engine::input_handling::console_input_handler::ConsoleInputHandler;
use engine::text_rendering::console_text_renderer::ConsoleTextRenderer;

use engine::core::GameApp;
use game::states::MainMenuState;
use std::process;

fn main() {
    // TODO: Config file with screen_width
    let text_renderer = ConsoleTextRenderer::new(100);
    let input_handler = ConsoleInputHandler::new();
    let mut app = GameApp::new(text_renderer, input_handler);

    if let Err(e) = app.run(MainMenuState::new()) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
