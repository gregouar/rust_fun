use engine::core::config::{Config, ConfigSection, ConfigSetting};
use engine::DynResult;

#[derive(ConfigSection)]
pub struct GameplayConfigSection {
    pub hard_mode: ConfigSetting<bool>,
}

#[derive(ConfigSection)]
pub struct WindowConfigSection {
    pub width: ConfigSetting<usize>,
    pub horizontal_separator: ConfigSetting<char>,
}

#[derive(Config)]
pub struct GameConfig {
    pub gameplay: GameplayConfigSection,
    pub window: WindowConfigSection,
}

impl GameConfig {
    pub fn new() -> Self {
        GameConfig {
            gameplay: GameplayConfigSection {
                hard_mode: ConfigSetting::new(false),
            },
            window: WindowConfigSection {
                width: ConfigSetting::new(100),
                horizontal_separator: ConfigSetting::new('-'),
            },
        }
    }
}
