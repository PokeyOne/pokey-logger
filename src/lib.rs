#![allow(dead_code)]

#[cfg(test)]
mod tests;

mod time;
mod color;
use color::{TermColor::*, colorize};

use std::fmt::Display;
use std::str::FromStr;
use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

lazy_static!(
    /// The global logger.
    pub static ref LOGGER: Logger = Logger::new();
);

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::LOGGER.debug(&format!($($arg)*));
    }
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::LOGGER.info(&format!($($arg)*));
    }
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::LOGGER.warn(&format!($($arg)*));
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::LOGGER.error(&format!($($arg)*));
    }
}

// TODO: Scoped references that are basically references for certain files and
//       store the scope name and point to the logger. Then in that scope
//       the scoped ref can be used and it will just log with the scope name
//       at the beginning of the log message.
//
//       For example: let scoped_ref = LOGGER.scope("my_scope");
//                    scoped_ref.debug("Hello world!");
//
//       This would log "[DEBUG][my_scope] Hello world!"
#[derive(Debug)]
pub struct Logger {
    level: Mutex<Level>,
    color: AtomicBool
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Level {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    None = 4
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

impl Logger {
    fn new() -> Logger {
        Logger {
            level: Mutex::new(Level::Debug),
            color: AtomicBool::new(true)
        }
    }

    pub fn set_level(&self, level: Level) {
        *self.level.lock().unwrap() = level;
    }

    pub fn get_level(&self) -> Level {
        *self.level.lock().unwrap()
    }

    // TODO: I don't know enough about the ordering of the atomic bool to
    //       make an educated decision of the method to use. It is only ever
    //       set realistically a handful of times so probably not super
    //       important.
    pub fn set_color(&self, color: bool) {
        self.color.store(color, Ordering::Relaxed);
    }

    // TODO: use this
    pub fn get_color(&self) -> bool {
        self.color.load(Ordering::Relaxed)
    }

    // TODO: For now it just prints to stdout. In the future it should be
    //       able to print to a file and should do stuff asynchronously.
    pub fn debug(&self, message: &str) {
        if self.get_level() <= Level::Debug {
            if self.get_color() {
                println!("{} {}", colorize(Cyan, "[DEBUG]"), message);
            } else {
                println!("[DEBUG] {}", message);
            }
        }
    }

    pub fn info(&self, message: &str) {
        if self.get_level() <= Level::Info {
            if self.get_color() {
                println!("{} {}", colorize(Green, "[INFO]"), message);
            } else {
                println!("[INFO] {}", message);
            }
        }
    }

    pub fn warn(&self, message: &str) {
        if self.get_level() <= Level::Warn {
            if self.get_color() {
                println!("{} {}", colorize(Yellow, "[WARN]"), message);
            } else {
                println!("[WARN] {}", message);
            }
        }
    }

    pub fn error(&self, message: &str) {
        if self.get_level() <= Level::Error {
            if self.get_color() {
                println!("{} {}", colorize(Red, "[ERROR]"), message);
            } else {
                println!("[ERROR] {}", message);
            }
        }
    }
}

pub fn set_level(level: Level) {
    LOGGER.set_level(level);
}

pub fn set_color(color: bool) {
    LOGGER.set_color(color);
}

