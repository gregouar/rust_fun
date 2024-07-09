use engine::input_handling::console_input_handler::ConsoleInputHandler;
use engine::text_rendering::console_text_renderer::ConsoleTextRenderer;

use engine::core::GameApp;
use game::config;
use game::states::MainMenuState;
use std::process;

fn main() {
    let mut game_config = config::GameConfig::new();
    game_config.init_from_file("config.ini");

    // TODO: validation of setting (size >= 50 etc)
    let width_setting = game_config.get_setting(config::GameConfigSettings::Window(
        config::WindowSettings::Width,
    ));
    let mut window_width: usize = width_setting.read_value().unwrap();
    if window_width < 50 {
        window_width = width_setting.read_default_value().unwrap();
    }
    let text_renderer = ConsoleTextRenderer::new(window_width);
    let input_handler = ConsoleInputHandler::new();
    let mut app = GameApp::new(text_renderer, input_handler);

    if let Err(e) = app.run(MainMenuState::new()) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
