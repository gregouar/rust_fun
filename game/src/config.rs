use engine::core::config::{Config, ConfigSetting};

trait ToStr {
    fn to_str(&self) -> &'static str;
}

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

impl ToStr for WindowSettings {
    fn to_str(&self) -> &'static str {
        match self {
            WindowSettings::Width => "width",
        }
    }
}

fn section_enum_to_str(section: GameConfigSettings) -> &'static str {
    match section {
        GameConfigSettings::Window(_) => "window",
        GameConfigSettings::Graphics(_) => "graphics",
        GameConfigSettings::Sound(_) => "sound",
    }
}

fn setting_enum_to_str(section: GameConfigSettings) -> (&'static str, &'static str) {
    section_str= section_enum_to_str(section)
    match section {
        GameConfigSettings::Window(setting) => (
            "window",
            match setting {
                WindowSettings::Width => "width",
            },
        ),
        GameConfigSettings::Graphics(_) => ("graphics", ""),
        GameConfigSettings::Sound(_) => ("sound", ""),
    }
}
pub struct GameConfig {
    config: Config,
}

impl GameConfig {
    pub fn new() -> GameConfig {
        let mut config = Config::new();

        let window_section = config.add_or_get_section("window");
        window_section.add_or_get_setting("width", "100");
        let graphics_section = config.add_or_get_section("graphics");
        let sound_section = config.add_or_get_section("sound");

        GameConfig { config }
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

    // Yeah, thats not actually inheritance... need to have new create a GameConfig and store the actual config
    pub fn get_setting(&self, setting: GameConfigSettings) -> &ConfigSetting {
        let (section_name, setting_name) = setting_enum_to_str(setting);
        self.config.get_setting(section_name, setting_name).unwrap()
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
