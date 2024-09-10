use crate::DynResult;
use std::fmt;
use std::str::FromStr;

pub use engine_derive::Config;
pub use engine_derive::ConfigSection;

pub trait SettingType: FromStr + ToString + Clone {}
impl<T: FromStr + ToString + Clone> SettingType for T {}

/// ConfigSetting ///
pub struct ConfigSetting<T: SettingType> {
    pub value: T,
    default_value: T,
}

impl<T: SettingType> ConfigSetting<T> {
    pub fn new(value: T) -> Self {
        ConfigSetting::<T> {
            default_value: value.clone(),
            value: value,
        }
    }

    pub fn reset_to_default(&mut self) {
        self.value = self.default_value.clone();
    }
}

/// ConfigSection ///

pub trait ConfigSection: fmt::Display {}

/// Config ///

pub trait Config {
    fn save_to_file(&self, filepath: &str) -> DynResult;
    fn save_to_string(&self) -> DynResult<String>;
}
