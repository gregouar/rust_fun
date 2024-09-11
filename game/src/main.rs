use engine::input_handling::console_input_handler::ConsoleInputHandler;
use engine::text_rendering::console_text_renderer::ConsoleTextRenderer;

use engine::core::config::Config;

use engine::core::GameApp;
use game::config;
use game::states::MainMenuState;
use std::process;

fn main() {
    let mut game_config = config::GameConfig::new();
    if let Err(e) = game_config.load_from_file("config.ini") {
        eprintln!("Failed to load configuration file: {}", e);
    }

    let text_renderer = ConsoleTextRenderer::new(
        game_config.window.width.value,
        game_config.window.horizontal_separator.value,
    );
    let input_handler = ConsoleInputHandler::new();
    let mut app = GameApp::new(text_renderer, input_handler);

    if let Err(e) = app.run(MainMenuState::new()) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    if let Err(e) = game_config.save_to_file("config.ini") {
        eprintln!("Failed to save configuration file: {}", e);
    }
}
