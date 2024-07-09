use engine::input_handling::console_input_handler::ConsoleInputHandler;
use engine::text_rendering::console_text_renderer::ConsoleTextRenderer;

use engine::core::GameApp;
use game::config;
use game::states::MainMenuState;
use std::process;

fn main() {
    let mut game_config = config::GameConfig::new();
    let config_path = "config.ini";
    match game_config.load_from_file(config_path) {
        Ok(_) => (),
        Err(_) => game_config
            .save_to_file(config_path)
            .expect("Couldn't save config file."),
    };

    // TODO: validation of setting (size >= 50 etc)
    let text_renderer = ConsoleTextRenderer::new(
        game_config
            .read_setting_value(config::WINDOW_SECTION, "width")
            .unwrap(),
    );
    let input_handler = ConsoleInputHandler::new();
    let mut app = GameApp::new(text_renderer, input_handler);

    if let Err(e) = app.run(MainMenuState::new()) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
