//! This module contains the [`Level`] object, which is essentially a severity
//! level for log messages.
//!
//! It is used in various parts of the code to let the user looking at the log
//! know what kind of message they are looking at, as well as messages can
//! be filtered to only show above a certain level.

use crate::TermColor::{self, *};
use std::fmt::Display;
use std::str::FromStr;

#[cfg(feature = "config")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[cfg_attr(feature = "config", derive(Serialize, Deserialize))]
/// The log level.
///
/// This is used to determine which messages are logged. The higher the level,
/// the more important the message. The order is:
/// * `Debug`
/// * `Info`
/// * `Warn`
/// * `Error`
/// * `None`
///
/// The `None` level is used to disable logging.
///
/// To set the log level, use the `set_level` method on the logger.
/// ```
/// use pokey_logger::{Level::Info, LOGGER};
/// // This will set the log level to `Info`, and not display any debug messages.
/// LOGGER.set_level(Info);
/// ```
pub enum Level {
    /// Basic messages that shouldn't be shown to an end user in most cases.
    Debug = 0,
    /// Informational messages that may or may not be important.
    Info = 1,
    /// Something has gone wrong, but the program can still continue.
    Warn = 2,
    /// Something has gone wrong and failed. The program may keep running, but
    /// the error will most-likely not fix itself.
    Error = 3,
    /// Used for filtering to show no log messages.
    None = 4
}

impl Level {
    /// Returns the [`TermColor`] associated with the level.
    pub fn get_color(&self) -> TermColor {
        match self {
            Level::Debug => Cyan,
            Level::Info => Green,
            Level::Warn => Yellow,
            Level::Error => Red,
            Level::None => Reset
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Level::Debug => write!(f, "DEBUG"),
            Level::Info => write!(f, "INFO"),
            Level::Warn => write!(f, "WARN"),
            Level::Error => write!(f, "ERROR"),
            Level::None => write!(f, "NONE")
        }
    }
}

impl FromStr for Level {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEBUG" => Ok(Level::Debug),
            "INFO" => Ok(Level::Info),
            "WARN" => Ok(Level::Warn),
            "ERROR" => Ok(Level::Error),
            "NONE" => Ok(Level::None),
            _ => Err(format!("Invalid level: {}", s))
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Level::Info
    }
}
