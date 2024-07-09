use engine::input_handling::console_input_handler::ConsoleInputHandler;
use engine::text_rendering::console_text_renderer::ConsoleTextRenderer;

use engine::core::GameApp;
use game::config;
use game::states::MainMenuState;
use std::process;

fn main() {
    let mut config = config::GameConfig::new();
    let config_path = "config.ini";
    match config.load_from_file(config_path) {
        Ok(_) => (),
        Err(_) => config
            .save_to_file(config_path)
            .expect("Couldn't save config file."),
    };

    let text_renderer =
        ConsoleTextRenderer::new(config.read_setting_value("window", "width").unwrap());
    let input_handler = ConsoleInputHandler::new();
    let mut app = GameApp::new(text_renderer, input_handler);

    if let Err(e) = app.run(MainMenuState::new()) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
