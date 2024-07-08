use std::collections::HashMap;
use std::error;
use std::error::Error;
use std::fmt;
use std::fmt::Write;
use std::fs;
use std::str::FromStr;

const ROOT_SECTION: &str = "root";

#[derive(Debug)]
pub struct MissingSettingError<'a> {
    setting_name: &'a str,
}

impl<'a> fmt::Display for MissingSettingError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Missing setting '{}'", self.setting_name)
    }
}

impl<'a> error::Error for MissingSettingError<'a> {}

/// Config ///
pub struct Config {
    sections: HashMap<String, ConfigSection>,
}

fn parse_section_name(raw_str: &str) -> Option<String> {
    if raw_str.starts_with('[') && raw_str.ends_with(']') {
        let mut c = raw_str.chars();
        c.next();
        c.next_back();
        if let Ok(section_name) = String::from_str(c.as_str()) {
            return Some(section_name);
        }
    }
    None
}

fn parse_setting(raw_str: &str) -> Option<(String, String)> {
    if let Some((setting_name_str, setting_value_str)) = raw_str.split_once('=') {
        return Some((
            String::from(setting_name_str.trim()),
            String::from(setting_value_str.trim()),
        ));
    }
    None
}

impl Config {
    pub fn new() -> Self {
        let mut config = Config {
            sections: HashMap::new(),
        };

        config.add_or_get_section(ROOT_SECTION);

        config
    }

    pub fn add_or_get_section(&mut self, section_name: &str) -> &mut ConfigSection {
        let section_key = section_name.to_lowercase();
        self.sections
            .entry(section_key)
            .or_insert_with(|| ConfigSection::new(section_name))
    }

    pub fn load_from_file(&mut self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let file_content = fs::read_to_string(filepath)?;
        self.load_from_string(&file_content)
    }

    pub fn load_from_string(&mut self, raw_string: &str) -> Result<(), Box<dyn Error>> {
        let mut cur_section = self.add_or_get_section(ROOT_SECTION);

        for string_content in raw_string.lines().map(|x| x.trim()) {
            if let Some(section_name) = parse_section_name(&string_content) {
                cur_section = self.add_or_get_section(&section_name)
            } else if let Some((setting_name, setting_value)) = parse_setting(&string_content) {
                let cur_setting = cur_section.add_or_get_setting(&setting_name, &setting_value); // TODO: Log warning when setting not found
                cur_setting.set_value(&setting_value);
            }
        }

        Ok(())
    }

    pub fn save_to_string(&self) -> Result<String, Box<dyn Error>> {
        let mut string_content = String::new();
        // TODO: Prevent write section title for root
        for (_, section) in &self.sections {
            string_content.write_str(&format!("{}\n", section))?;
        }
        Ok(string_content)
    }

    pub fn save_to_file(&self, filepath: &str) -> Result<(), Box<dyn Error>> {
        Ok(fs::write(filepath, self.save_to_string()?)?)
    }

    fn get_section(&self, section_name: &str) -> Option<&ConfigSection> {
        self.sections.get(&section_name.to_lowercase())
    }

    pub fn read_setting_value<T: FromStr>(
        &self,
        section_name: &str,
        setting_name: &str,
    ) -> Result<T, Box<dyn Error>> {
        if let Some(section) = self.get_section(section_name) {
            return section.read_setting_value::<T>(setting_name);
        }
        Err(Box::new(MissingSettingError { setting_name }))
    }
}

/// ConfigSection ///
pub struct ConfigSection {
    section_name: String,
    settings: HashMap<String, ConfigSetting>,
}

impl ConfigSection {
    fn new(section_name: &str) -> Self {
        ConfigSection {
            section_name: String::from(section_name),
            settings: HashMap::new(),
        }
    }

    pub fn add_or_get_setting(&mut self, setting_name: &str, value: &str) -> &mut ConfigSetting {
        let setting_key = setting_name.to_lowercase();
        self.settings
            .entry(setting_key)
            .or_insert_with(|| ConfigSetting::new(setting_name, value))
    }

    fn get_setting(&self, setting_name: &str) -> Option<&ConfigSetting> {
        self.settings.get(&setting_name.to_lowercase())
    }

    fn read_setting_value<T: FromStr>(&self, setting_name: &str) -> Result<T, Box<dyn Error>> {
        if let Some(setting) = self.get_setting(setting_name) {
            return Ok(setting.read_value::<T>()?);
        }
        Err(Box::new(MissingSettingError { setting_name }))
    }
}

impl fmt::Display for ConfigSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " [{}]\n", self.section_name)?;
        for (_, setting) in &self.settings {
            write!(f, " {}\n", setting)?;
        }
        Ok(())
    }
}

/// ConfigSetting ///

pub struct ConfigSetting {
    name: String,
    value: String,
    default_value: String,
}

impl ConfigSetting {
    fn new(name: &str, value: &str) -> Self {
        ConfigSetting {
            name: String::from(name),
            default_value: String::from(value),
            value: String::from(value),
        }
    }

    fn set_value(&mut self, value: &str) {
        self.value = String::from(value);
    }

    fn read_value<T: FromStr>(&self) -> Result<T, T::Err> {
        if let Ok(value) = self.value.parse::<T>() {
            return Ok(value);
        }
        // LOG WARNING
        self.default_value.parse::<T>()
    }
}

impl fmt::Display for ConfigSetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}
