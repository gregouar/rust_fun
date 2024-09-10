use engine::input_handling::console_input_handler::ConsoleInputHandler;
use engine::text_rendering::console_text_renderer::ConsoleTextRenderer;

use engine::core::config2::Config;

use engine::core::GameApp;
use game::config;
use game::config2;
use game::states::MainMenuState;
use std::fmt::Write;
use std::process;

fn main() {
    let mut game_config2 = config2::GameConfig::new();
    println!("{}", game_config2.save_to_string().unwrap());
    game_config2.window.width.value = 42;
    println!("{}", game_config2.save_to_string().unwrap());

    // let mut game_config = config::GameConfig::new();
    // game_config.init_from_file("config.ini");

    // // WISH:
    // // let mut window_width = game_config.window.width.get_value()

    // // TODO: validation of setting (size >= 50 etc)
    // let width_setting = game_config.get_setting(config::GameConfigSettings::Window(
    //     config::WindowSettings::Width,
    // ));
    // let mut window_width: usize = width_setting.read_value().unwrap();
    // if window_width < 50 {
    //     window_width = width_setting.read_default_value().unwrap();
    // }
    // let text_renderer = ConsoleTextRenderer::new(window_width);
    // let input_handler = ConsoleInputHandler::new();
    // let mut app = GameApp::new(text_renderer, input_handler);

    // if let Err(e) = app.run(MainMenuState::new()) {
    //     eprintln!("Application error: {e}");
    //     process::exit(1);
    // }
}
