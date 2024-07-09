use engine::core::config::Config;

pub struct GameConfig;

pub const WINDOW_SECTION: &str = "window";

impl GameConfig {
    pub fn new() -> Config {
        let mut config = Config::new();

        let window_section = config.add_or_get_section(WINDOW_SECTION);
        window_section.add_or_get_setting("width", "100");
        // let graphics_section = config.add_or_get_section("graphics");
        // let sound_section = config.add_or_get_section("sound");

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_parsing() {
        let config = GameConfig::new();

        config
            .read_setting_value::<usize>(WINDOW_SECTION, "width")
            .unwrap();
    }
}
