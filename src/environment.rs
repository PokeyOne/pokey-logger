//! This module handles environment variable interactions.
//!
//! This module is not included unles the `env` feature is enabled.

use super::Logger;
use crate::Level;
use std::env;

/// Apply the environment variable configuration.
///
/// Usually it will be easier to call the `load_env_vars` method on the Logger
/// struct instead of this.
///
/// # Variables
///
/// | Name       | Type    | Description |
/// |------------|---------|-------------|
/// | `PL_COLOR` | boolean | Whether or not to color the terminal output |
/// | `PL_FILE_COLOR` | boolean | Whether or not to color the file output |
/// | `PL_SHOW_TIME` | boolean | Whether or not to show a time stamp on messages |
/// | `PL_LEVEL` | debug,info,warn,error,none | Logging level |
/// | `PL_FILE` | string | Log file path |
///
/// For boolean flags, if set to "true", they will be true; if not set, then
/// they will have no effect either way; and if they are set to anything other
/// than "true", they will be "false".
pub fn configure(logger: &Logger) {
    match env::var("PL_COLOR") {
        Ok(val) if val == "true" => logger.set_color(true),
        Ok(_) => logger.set_color(false),
        // Unset, or invalid
        _ => {}
    };

    #[cfg(feature = "log_files")]
    match env::var("PL_FILE_COLOR") {
        Ok(val) if val == "true" => logger.set_log_file_color(true),
        Ok(_) => logger.set_log_file_color(false),
        // Unset, or invalid
        _ => {}
    };

    #[cfg(feature = "time")]
    match env::var("PL_SHOW_TIME") {
        Ok(val) if val == "true" => logger.set_should_show_time(true),
        Ok(_) => logger.set_should_show_time(false),
        // Unset, or invalid
        _ => {}
    };

    match env::var("PL_LEVEL") {
        Ok(val) => match val.to_lowercase().as_ref() {
            "debug" => logger.set_level(Level::Debug),
            "info" => logger.set_level(Level::Info),
            "warn" => logger.set_level(Level::Warn),
            "error" => logger.set_level(Level::Error),
            "none" => logger.set_level(Level::None),
            _ => {}
        },
        // Unset, or invalid
        _ => {}
    };

    #[cfg(feature = "log_files")]
    match env::var("PL_FILE") {
        Ok(val) => {
            logger.set_log_path(&val);
        }
        // Unset, or invalid
        _ => {}
    };
}
