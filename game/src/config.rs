use engine::core::config::{Config, ConfigSetting};

pub enum WindowSettings {
    Width,
}
pub enum GraphicsSettings {}
pub enum SoundSettings {}
pub enum GameConfigSettings {
    Window(WindowSettings),
    Graphics(GraphicsSettings),
    Sound(SoundSettings),
}

fn setting_enum_to_str(section: GameConfigSettings) -> (&'static str, &'static str) {
    match section {
        GameConfigSettings::Window(setting) => (
            "window",
            match setting {
                WindowSettings::Width => "width",
            },
        ),
        GameConfigSettings::Graphics(setting) => ("graphics", match setting {}),
        GameConfigSettings::Sound(setting) => ("sound", match setting {}),
    }
}

pub struct GameConfig {
    config: Config,
}

impl GameConfig {
    pub fn new() -> GameConfig {
        let mut game_config = GameConfig {
            config: Config::new(),
        };
        game_config.add_setting(GameConfigSettings::Window(WindowSettings::Width), "100");
        let graphics_section = game_config.config.add_or_get_section("graphics");
        let sound_section = game_config.config.add_or_get_section("sound");
        game_config
    }

    pub fn init_from_file(&mut self, config_filepath: &str) {
        match self.config.load_from_file(config_filepath) {
            Ok(_) => (),
            Err(_) => self
                .config
                .save_to_file(config_filepath)
                .expect("Couldn't save config file."),
        }
    }

    pub fn get_setting(&self, setting: GameConfigSettings) -> &ConfigSetting {
        let (section_name, setting_name) = setting_enum_to_str(setting);
        self.config.get_setting(section_name, setting_name).unwrap()
    }

    fn add_setting(&mut self, setting: GameConfigSettings, value: &str) {
        let (section_name, setting_name) = setting_enum_to_str(setting);
        self.config
            .add_or_get_section(section_name)
            .add_or_get_setting(setting_name, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_settings_parsing() {
        let config = GameConfig::new();

        // TODO: Macro ?
        config
            .get_setting(GameConfigSettings::Window(WindowSettings::Width))
            .read_default_value::<usize>()
            .unwrap();
    }
}
