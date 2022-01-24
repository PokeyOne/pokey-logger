use std::io;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::Level;

// TODO: There should be different environments. All of these fields should be
//       under the 'default' heading, then allow a set of environments to be
//       defined.
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    #[serde(default = "Level::default")]
    pub level: Level,
    #[serde(default = "default_true")]
    pub color: bool,
    #[serde(default = "default_true")]
    pub time_stamp: bool,
    #[serde(default = "default_false")]
    pub file_color: bool,
    pub log_file_path: Option<String>
}

#[derive(Debug)]
pub enum ConfigFileLoadError {
    IoError(io::Error),
    YamlError(serde_yaml::Error)
}

impl ConfigFile {
    pub fn load(from_path: &str) -> Result<Self, ConfigFileLoadError> {
        let file = fs::read_to_string(from_path).map_err(ConfigFileLoadError::IoError)?;
        let config: ConfigFile = serde_yaml::from_str(&file).map_err(ConfigFileLoadError::YamlError)?;
        Ok(config)
    }
}

/// Used to set default values for the config file
fn default_true() -> bool {
    true
}

/// Used to set default values for the config file
fn default_false() -> bool {
    false
}