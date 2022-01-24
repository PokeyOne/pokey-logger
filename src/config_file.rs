use std::io;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::Level;

// TODO: There should be different environments. All of these fields should be
//       under the 'default' heading, then allow a set of environments to be
//       defined.
/// Structure for the configuration file.
///
/// This is a direct mapping of what is defined in the yaml file.
///
/// # Examples
/// ```
/// use pokey_logger::LOGGER;
///
/// LOGGER.load_config_file("examples/full_usage/config.yml").unwrap();
/// // Now the logger is configured.
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    /// The level of logging to use.
    #[serde(default = "Level::default")]
    pub level: Level,
    /// Whether or not to include colors in the terminal output.
    #[serde(default = "default_true")]
    pub color: bool,
    /// Whether or not to include timestamps in the terminal output.
    #[serde(default = "default_true")]
    pub time_stamp: bool,
    /// Whether or not to include colors in the file output.
    #[serde(default = "default_false")]
    pub file_color: bool,
    /// The path to the file to log to. If none, then no file logging will be
    /// done.
    pub log_file_path: Option<String>
}

/// An error in loading a configuration file.
#[derive(Debug)]
pub enum ConfigFileLoadError {
    /// Something went wrong while reading the file itself.
    IoError(io::Error),
    /// Something went wrong while parsing the yaml file. This can the actual
    /// syntax itself or it could be a problem with incorrect fields or values.
    YamlError(serde_yaml::Error)
}

impl ConfigFile {
    /// Load a configuration file from the given path.
    ///
    /// Will return an error if the file cannot be read or parsed, or if the
    /// structure of the file is incorrect.
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