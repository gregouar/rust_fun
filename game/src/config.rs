use engine::core::config::Config;

pub enum GameConfigSection {
    Window,
    Graphics,
    Sound,
}

pub struct GameConfig;

impl GameConfig {
    pub fn new() -> Config {
        let mut config = Config::new();

        let window_section = config.add_or_get_section("window");
        window_section.add_or_get_setting("width", "100");
        // let graphics_section = config.add_or_get_section("graphics");
        // let sound_section = config.add_or_get_section("sound");

        config
    }
}
