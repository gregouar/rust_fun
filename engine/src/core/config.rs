use crate::DynResult;
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
    setting_name: &'a str, // Could also use a String for simplicity
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
        let new_section_id = self.sections.len();
        self.sections
            .entry(section_key)
            .or_insert_with(|| ConfigSection::new(new_section_id, section_name))
    }

    pub fn load_from_file(&mut self, filepath: &str) -> DynResult {
        let file_content = fs::read_to_string(filepath)?;
        self.load_from_string(&file_content)
    }

    pub fn load_from_string(&mut self, raw_string: &str) -> DynResult {
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

    pub fn save_to_string(&self) -> DynResult<String> {
        let mut string_content = String::new();

        // TODO: Prevent write section title for root
        let mut sorted_vec: Vec<&ConfigSection> = self.sections.values().collect();
        sorted_vec.sort_by(|&x, &y| x.get_id().cmp(&y.get_id()));
        for section in sorted_vec {
            string_content.write_str(&format!("{}\n", section))?;
        }

        Ok(string_content)
    }

    pub fn save_to_file(&self, filepath: &str) -> DynResult {
        Ok(fs::write(filepath, self.save_to_string()?)?)
    }

    fn get_section(&self, section_name: &str) -> Option<&ConfigSection> {
        self.sections.get(&section_name.to_lowercase())
    }

    pub fn read_setting_value<'a, T: FromStr>(
        &'a self,
        section_name: &str,
        setting_name: &'a str,
    ) -> Result<T, Box<dyn Error + 'a>> {
        if let Some(section) = self.get_section(section_name) {
            return section.read_setting_value::<T>(setting_name);
        }
        Err(Box::new(MissingSettingError { setting_name }))
    }

    pub fn get_setting(&self, section_name: &str, setting_name: &str) -> Option<&ConfigSetting> {
        if let Some(section) = self.get_section(section_name) {
            return section.get_setting(setting_name);
        }
        None
    }
}

/// ConfigSection ///
pub struct ConfigSection {
    section_id: usize,
    section_name: String,
    settings: HashMap<String, ConfigSetting>,
}

impl ConfigSection {
    fn new(section_id: usize, section_name: &str) -> Self {
        ConfigSection {
            section_id,
            section_name: String::from(section_name),
            settings: HashMap::new(),
        }
    }

    pub fn add_or_get_setting(&mut self, setting_name: &str, value: &str) -> &mut ConfigSetting {
        let setting_key = setting_name.to_lowercase();
        let new_setting_id = self.settings.len();
        self.settings
            .entry(setting_key)
            .or_insert_with(|| ConfigSetting::new(new_setting_id, setting_name, value))
    }

    pub fn get_setting(&self, setting_name: &str) -> Option<&ConfigSetting> {
        self.settings.get(&setting_name.to_lowercase())
    }

    fn read_setting_value<'a, T: FromStr>(
        &'a self,
        setting_name: &'a str,
    ) -> Result<T, Box<dyn Error + 'a>> {
        if let Some(setting) = self.get_setting(setting_name) {
            return setting.read_value::<T>();
        }
        Err(Box::new(MissingSettingError { setting_name }))
    }

    fn get_id(&self) -> usize {
        self.section_id
    }
}

impl fmt::Display for ConfigSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.section_name != ROOT_SECTION {
            write!(f, "[{}]\n", self.section_name)?;
        }
        let mut sorted_vec: Vec<&ConfigSetting> = self.settings.values().collect();
        sorted_vec.sort_by(|&x, &y| x.get_id().cmp(&y.get_id()));
        for setting in sorted_vec {
            write!(f, "{}\n", setting)?;
        }
        Ok(())
    }
}

/// ConfigSetting ///
pub struct ConfigSetting {
    id: usize,
    name: String,
    value: String,
    default_value: String,
}

impl ConfigSetting {
    fn new(id: usize, name: &str, value: &str) -> Self {
        ConfigSetting {
            id,
            name: String::from(name),
            default_value: String::from(value),
            value: String::from(value),
        }
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = String::from(value);
    }

    pub fn read_value<'a, T: FromStr>(&'a self) -> Result<T, Box<dyn Error + 'a>> {
        if let Ok(value) = self.value.parse::<T>() {
            return Ok(value);
        }
        // LOG WARNING

        self.read_default_value::<T>()
    }

    pub fn read_default_value<'a, T: FromStr>(&'a self) -> Result<T, Box<dyn Error + 'a>> {
        if let Ok(value) = self.default_value.parse::<T>() {
            return Ok(value);
        }

        Err(Box::new(MissingSettingError {
            setting_name: &self.name,
        }))
    }

    fn get_id(&self) -> usize {
        self.id
    }
}

impl fmt::Display for ConfigSetting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_config() {
        let mut config = Config::new();

        let section1 = config.add_or_get_section("section1");
        section1.add_or_get_setting("setting1", "default1");

        let section2 = config.add_or_get_section("section2");
        section2.add_or_get_setting("setting1", "42");
        section2.add_or_get_setting("setting2", "default2");

        config
            .load_from_string(
                "
        setting1 = rootvalue
        
        [section1]
        setting1 = test
        setting2 = 32

        nonparsable

        [section2]
        setting1 = 2

        [section1]
        setting3 = 3
        
        ",
            )
            .unwrap();

        assert_eq!(
            config
                .read_setting_value::<String>("root", "setting1")
                .unwrap(),
            "rootvalue"
        );

        assert_eq!(
            config
                .read_setting_value::<String>("section1", "setting1")
                .unwrap(),
            "test"
        );
        assert_eq!(
            config
                .read_setting_value::<i32>("section1", "setting2")
                .unwrap(),
            32
        );
        assert_eq!(
            config
                .read_setting_value::<i32>("section1", "setting3")
                .unwrap(),
            3
        );
        assert_eq!(
            config
                .read_setting_value::<usize>("section2", "setting1")
                .unwrap(),
            2
        );
        assert_eq!(
            config
                .read_setting_value::<String>("section2", "setting2")
                .unwrap(),
            "default2"
        );

        assert_eq!(config.save_to_string().unwrap(), "setting1 = rootvalue\n\n[section1]\nsetting1 = test\nsetting2 = 32\nsetting3 = 3\n\n[section2]\nsetting1 = 2\nsetting2 = default2\n\n")
    }
}
