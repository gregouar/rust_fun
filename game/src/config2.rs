use engine::core::config2::{Config, ConfigSection, ConfigSetting};
use engine::DynResult;
use std::fmt;
use std::fmt::Write;
use std::fs;

#[derive(ConfigSection)]
pub struct WindowConfig {
    pub width: ConfigSetting<usize>,
}

#[derive(Config)]
pub struct GameConfig {
    pub window: WindowConfig,
}

impl GameConfig {
    pub fn new() -> Self {
        GameConfig {
            window: WindowConfig {
                width: ConfigSetting::new(50),
            },
        }
    }
}
