use crate::DynResult;
use std::fmt;
use std::str::FromStr;

pub use engine_derive::Config;
pub use engine_derive::ConfigSection;
use std::fs;

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

    pub fn load_value_from_string(&mut self, raw_string: &str) {
        if let Ok(value) = T::from_str(raw_string) {
            self.value = value;
        }
    }
}

/// ConfigSection ///

pub trait ConfigSection: fmt::Display {
    fn load_setting_from_string(&mut self, raw_string: &str);
}

/// Config ///

pub trait Config {
    fn load_from_string(&mut self, raw_string: &str) -> DynResult;
    fn load_from_file(&mut self, filepath: &str) -> DynResult {
        let file_content = fs::read_to_string(filepath)?;
        self.load_from_string(&file_content)
    }

    fn save_to_string(&self) -> DynResult<String>;
    fn save_to_file(&self, filepath: &str) -> DynResult {
        Ok(fs::write(filepath, self.save_to_string()?)?)
    }
}

pub fn parse_section_name(raw_string: &str) -> Option<String> {
    if raw_string.starts_with('[') && raw_string.ends_with(']') {
        let mut c = raw_string.chars();
        c.next();
        c.next_back();
        if let Ok(section_name) = String::from_str(c.as_str()) {
            return Some(section_name);
        }
    }
    None
}

pub fn parse_setting(raw_string: &str) -> Option<(String, String)> {
    if let Some((setting_name_str, setting_value_str)) = raw_string.split_once('=') {
        return Some((
            String::from(setting_name_str.trim()),
            String::from(setting_value_str.trim()),
        ));
    }
    None
}
