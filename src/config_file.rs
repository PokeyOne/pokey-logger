use std::io;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::Level;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    level: Level
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