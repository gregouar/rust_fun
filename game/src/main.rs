use engine::text_rendering::console_text_renderer::ConsoleTextRenderer;

use engine::core::GameApp;
use game::states::MainMenuState;
use std::process;

fn main() {
    // TODO: Config file with screen_width
    let text_renderer = ConsoleTextRenderer::new(100);
    let mut app = GameApp::new(text_renderer);

    if let Err(e) = app.run(MainMenuState::new()) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
