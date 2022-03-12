//! A simple logging library for Rust.
//!
//! ## Usage
//! First, the library must be added to the project's `Cargo.toml` file.
//! ```toml
//! pokey_logger = "0.2.0"
//! ```
//! or to get the latest and greatest
//! ```toml
//! pokey_logger = { git = "https://github.com/PokeyOne/pokey-logger" }
//! ```
//! For more advanced methods see [the Cargo documentation on specifying dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
//!
//! ### Usage in Rust
//!
//! For complete instructions, see the rustdoc documentation. Below is a simple
//! example.
//!
//! This is an example of logging some messages. It is assumed that before this
//! that the `debug!` macro, the `Level` type, and the `LOGGER` constant have been
//! imported.
//!
//! ```rust
//! use pokey_logger::{Level, LOGGER, debug, warn};
//!
//! fn main() {
//!     // Optionally you can configure the output log level and whether or not colours
//!     // are shown in the terminal
//!     LOGGER.set_color(true);
//!     LOGGER.set_level(Level::Debug);
//!     #[cfg(feature = "log_files")]
//!     if !LOGGER.set_log_path("logs/server.log") {
//!         warn!("Could not set log path");
//!     }
//!
//!     // This will print a debug message using the `debug!` macro. The available macros
//!     // are debug, info, warn, and error.
//!     // The usage is exactly the same as a format! or println! macro.
//!     debug!("Some message with the number {} in it", 4);
//! }
//! ```
//!
//! As of version 0.2.0 of the library a log file can be added. It should be noted
//! that the library will **never** create directories, but it will create log files
//! if they don't exist. For example in the above example program, the logger would
//! not create the logs directory, but if the logs directory existed and the file
//! did not, it would be able to create the `server.log` file.
//!
//! It is also valuable to note that `LOGGER` is a global static instance of the
//! `Logger`. It is thread safe to use, but one should be careful about configuring
//! its settings from multiple threads. If you would like separate configurations
//! and instances, the `Logger` struct itself can be instantiated and passed around
//! as the developer sees fit.
//!
//! # Features
//!
//! There are a few features that can be turned off when using the crate to
//! build a smalled binary.
//!
//! ## Default Features
//!
//! - **time** - Use `chrono` to put the time at the begining of messages.
//! - **log_files** - Allow log files to be saved

#![allow(dead_code)]
// Allow needless doctest main function because example above makes more sense
// with the main function.
#![allow(clippy::needless_doctest_main)]

#[cfg(test)]
mod tests;
#[macro_use]
pub mod logging_macros;
pub mod color;
#[cfg(feature = "log_files")]
pub mod existing_log_handler;
#[cfg(feature = "time")]
pub mod time;

mod config_file;
mod level; // not public because level is reexported
mod log_message;

pub use config_file::ConfigFileLoadError;
pub use level::Level;

#[cfg(feature = "log_files")]
use crate::existing_log_handler::ExistingLogHandler;
#[cfg(feature = "log_files")]
use std::fs::File;
#[cfg(feature = "log_files")]
use std::io::{prelude::*, BufWriter};
#[cfg(feature = "log_files")]
use std::path::PathBuf;

use color::TermColor;
use config_file::ConfigFile;
use lazy_static::lazy_static;
use log_message::LogMessage;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

lazy_static!(
    /// The global logger.
    pub static ref LOGGER: Logger = Logger::new();
);

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
/// The logger struct. This is the main struct that is used to log messages.
///
/// It is recommended that you use the `LOGGER` global static instance of this
/// struct for most logging, but this can be used to create separate instances
/// if multiple configurations are needed.
///
/// # Configuration
/// All configuration is done through the `set_*` methods. These methods do
/// not require mutable access to the logger because all of the settings are
/// behind a mutex or atomic boolean.
/// ```rust
/// use pokey_logger::{Level, LOGGER, warn, ConfigFileLoadError};
/// // Only log debug and above. default is info
/// LOGGER.set_level(Level::Debug);
/// // Turn off colors. default is true
/// LOGGER.set_color(false);
/// // Turn off the timestamp. default is true
/// LOGGER.set_should_show_time(false);
/// // Set the log path. default is none. See remove_log_path
/// #[cfg(feature = "log_files")]
/// if !LOGGER.set_log_path("logs/server.log") {
///     warn!("Could not set log path");
/// }
/// // Whether or not to put colour code in the log file. default is false
/// #[cfg(feature = "log_files")]
/// LOGGER.set_log_file_color(true);
/// // Or even load a configuration file
/// #[cfg(feature = "log_files")]
/// if let Err(e) = LOGGER.load_config_file("config/logger.yml") {
///    warn!("Could not load config file: {e:?}");
/// }
/// ```
pub struct Logger {
    level: Mutex<Level>,
    color: AtomicBool,
    #[cfg(feature = "log_files")]
    log_file_color: AtomicBool,
    show_time: AtomicBool,
    #[cfg(feature = "log_files")]
    log_path: Mutex<Option<PathBuf>>,
    #[cfg(feature = "log_files")]
    log_writer: Mutex<Option<BufWriter<File>>>,
    #[cfg(feature = "log_files")]
    existing_log_handler: Mutex<ExistingLogHandler>,
    // TODO: Make timestamp_format feature dependent
    timestamp_format: Mutex<Option<String>>
}

impl Logger {
    /// Create a new Logger instance with all default settings. This never
    /// needs to be mutable because all settings are wrapped in an atomic
    /// or mutex reference.
    pub fn new() -> Logger {
        Logger {
            level: Mutex::new(Level::Debug),
            color: AtomicBool::new(true),
            #[cfg(feature = "log_files")]
            log_file_color: AtomicBool::new(false),
            show_time: AtomicBool::new(true),
            #[cfg(feature = "log_files")]
            log_path: Mutex::new(None),
            #[cfg(feature = "log_files")]
            log_writer: Mutex::new(None),
            #[cfg(feature = "log_files")]
            existing_log_handler: Mutex::new(ExistingLogHandler::Overwrite),
            timestamp_format: Mutex::new(None)
        }
    }

    /// Set the log level. Only logs with a level equal to or higher than the
    /// set level will be logged.
    pub fn set_level(&self, level: Level) {
        *self.level.lock().unwrap() = level;
    }

    /// Get the current log level.
    pub fn get_level(&self) -> Level {
        *self.level.lock().unwrap()
    }

    // TODO: I don't know enough about the ordering of the atomic bool to
    //       make an educated decision of the method to use. It is only ever
    //       set realistically a handful of times so probably not super
    //       important.
    /// Set whether or not the logger should use colors. True means use colors,
    /// false means don't use colors.
    pub fn set_color(&self, color: bool) {
        self.color.store(color, Ordering::Relaxed);
    }

    /// Get whether or not the logger should use colors. True means use colors,
    /// false means don't use colors.
    pub fn get_color(&self) -> bool {
        self.color.load(Ordering::Relaxed)
    }

    /// Set whether or not the logger should use colors in the log file. True
    /// means use colors, false means don't use colors.
    #[cfg(feature = "log_files")]
    pub fn set_log_file_color(&self, color: bool) {
        self.log_file_color.store(color, Ordering::Relaxed);
    }

    /// Get whether or not the logger should use colors in the log file. True
    /// means use colors, false means don't use colors.
    #[cfg(feature = "log_files")]
    pub fn get_log_file_color(&self) -> bool {
        self.log_file_color.load(Ordering::Relaxed)
    }

    /// Set how existing log files should be handled.
    #[cfg(feature = "log_files")]
    pub fn set_existing_log_handler(&self, handler: ExistingLogHandler) {
        *self.existing_log_handler.lock().unwrap() = handler;
    }

    /// Get how existing log files should be handled.
    #[cfg(feature = "log_files")]
    pub fn get_existing_log_handler(&self) -> ExistingLogHandler {
        *self.existing_log_handler.lock().unwrap()
    }

    /// Set whether or not the logger should show the timestamp. True means
    /// show the timestamp, false means don't show the timestamp.
    pub fn set_should_show_time(&self, show_time: bool) {
        self.show_time.store(show_time, Ordering::Relaxed);
    }

    /// Get whether or not the logger should show the timestamp. True means
    /// show the timestamp, false means don't show the timestamp.
    pub fn should_show_time(&self) -> bool {
        self.show_time.load(Ordering::Relaxed)
    }

    // TODO: Consider maybe just returning a Result<(), String> type that the
    //       caller can decide what to do with that reasoning. Or even a custom
    //       error enum that implements Display
    /// Set the path to log to.
    ///
    /// This method will never panic, but does return a boolean value of whether
    /// or not the path was actually set. The **path is not set** under the
    /// following conditions:
    /// 1. If the path does not exist.
    /// 1. If the path is a directory
    /// 1. If the file does not exist, and no permission to create it.
    /// In the event of the path not being set, false will be returned and an
    /// error message will be printed with the reasoning.
    ///
    /// The method **will not** create directories, but it **will** create files
    /// if they don't exist.
    ///
    /// # Returns
    /// The return value will be `true` if the path is successfully set, or
    /// `false` if could not set the path.
    #[cfg(feature = "log_files")]
    pub fn set_log_path(&self, path: &str) -> bool {
        let path_buf = PathBuf::from(path);
        self.remove_log_writer();

        // Create the file if it doesn't exist, but don't touch directory
        // structures.
        if !path_buf.exists() && File::create(path).is_err() {
            error!("Log path specified does not exist.");
            return false;
        }

        // Check that it is in fact a file.
        if !path_buf.is_file() {
            error!("Log path specified is not a file. Please specify a file.");
            return false;
        }

        *self.log_path.lock().unwrap() = Some(path_buf);

        true
    }

    /// Remove file logging.
    #[cfg(feature = "log_files")]
    pub fn remove_log_path(&self) {
        *self.log_path.lock().unwrap() = None;
        self.remove_log_writer();
    }

    /// Get the path to the file that the logger is logging to.
    #[cfg(feature = "log_files")]
    pub fn get_log_path(&self) -> Option<PathBuf> {
        (*self.log_path.lock().unwrap()).as_ref().cloned()
    }

    /// Get the format of the timestamp on log messages.
    pub fn get_timestamp_format(&self) -> Option<String> {
        let res = match self.timestamp_format.lock() {
            Ok(ref mut inner) => inner.clone(),
            Err(_) => return None
        };

        res
    }

    /// Set the format of the timestamp on log messages.
    pub fn set_timestamp_format(&self, value: Option<String>) {
        match self.timestamp_format.lock() {
            Ok(mut inner) => *inner = value,
            Err(_) => error!("Could not set timestamp format.")
        }
    }

    /// Set the file writer to write actual data to. This method should only
    /// be called internally.
    #[cfg(feature = "log_files")]
    fn set_log_writer(&self, buf_writer: BufWriter<File>) {
        *self.log_writer.lock().unwrap() = Some(buf_writer);
    }

    /// Remove the file writer. This method should only be called internally.
    #[cfg(feature = "log_files")]
    fn remove_log_writer(&self) {
        *self.log_writer.lock().unwrap() = None;
    }

    /// Whether or not the logger writer has already been set. If this method
    /// is false but log path is set, a log writer should be created.
    /// See [`set_log_writer_if_not_set`](#method.set_log_writer_if_not_set)
    #[cfg(feature = "log_files")]
    fn has_log_writer(&self) -> bool {
        if let Ok(lw) = self.log_writer.lock() {
            return lw.is_some();
        }

        false
    }

    /// If there is a log path set, but no log writer, create a log writer.
    #[cfg(feature = "log_files")]
    fn set_log_writer_if_not_set(&self) {
        if !self.has_log_writer() {
            if let Some(path) = self.get_log_path() {
                let file = match self.get_existing_log_handler().open_file(&path) {
                    Ok(f) => f,
                    Err(e) => {
                        error!("Could not open log file: {:?}", e);
                        self.remove_log_path();
                        return;
                    }
                };
                let buf_writer = BufWriter::new(file);
                self.set_log_writer(buf_writer);
            }
        }
    }

    #[cfg(feature = "log_files")]
    fn log_message_to_file(&self, log_message: &mut LogMessage) {
        // Write to file
        self.set_log_writer_if_not_set();
        if let Ok(ref mut log_writer) = self.log_writer.lock() {
            if log_writer.is_some() {
                let formatted_message = log_message.formatted(self.get_log_file_color());
                if let Err(e) = log_writer
                    .as_mut()
                    .unwrap()
                    .write(formatted_message.as_bytes())
                {
                    // Remove the writer and the path, then log an error
                    self.remove_log_writer();
                    self.remove_log_path();
                    self.error(&format!("Log file could not be written to: {e:?}"));
                }
            }
        }
    }

    #[cfg(not(feature = "log_files"))]
    fn log_message_to_file(&self, _msg: &mut LogMessage) {
        // Intentionally do nothing when the feature is not enabled
    }


    /// Actually write the log message to the file and stdout. Should only be
    /// called internally by the `debug`, `info`, `warn`, and `error` methods.
    fn log_message(&self, level: Level, message: &str) {
        let mut log_message = LogMessage::new(&self.prefix(), message, level);

        // Print to stdout
        print!("{}", log_message.formatted(self.get_color()));

        #[cfg(feature = "log_files")]
        self.log_message_to_file(&mut log_message);
    }

    // TODO: For now it just prints to stdout. In the future it should be
    //       able to print to a file and should do stuff asynchronously.
    /// Print a message to the log at the `debug` level. Will only print if the
    /// `debug` level is enabled.
    pub fn debug(&self, message: &str) {
        if self.get_level() <= Level::Debug {
            self.log_message(Level::Debug, message);
        }
    }

    /// Print a message to the log at the `info` level. Will only print if the
    /// `info` level is enabled.
    pub fn info(&self, message: &str) {
        if self.get_level() <= Level::Info {
            self.log_message(Level::Info, message);
        }
    }

    /// Print a message to the log at the `warn` level. Will only print if the
    /// `warn` level is enabled.
    pub fn warn(&self, message: &str) {
        if self.get_level() <= Level::Warn {
            self.log_message(Level::Warn, message);
        }
    }

    /// Print a message to the log at the `error` level. Will only print if the
    /// `error` level is enabled.
    pub fn error(&self, message: &str) {
        if self.get_level() <= Level::Error {
            self.log_message(Level::Error, message);
        }
    }

    /// The prefix to be added to all log messages. Currently this is just the
    /// timestamp, but in the future it could be something like the scope of
    /// the log message.
    #[cfg(feature = "time")]
    fn prefix(&self) -> String {
        if self.should_show_time() {
            time::current_time_box(self.get_timestamp_format())
        } else {
            "".to_string()
        }
    }

    /// The prefix to be added to all log messages. Currently this is just the
    /// timestamp, but in the future it could be something like the scope of
    /// the log message.
    #[cfg(not(feature = "time"))]
    fn prefix(&self) -> String {
        "".to_string()
    }

    /// Ensure all io buffers are cleared; usually before shutdown.
    pub fn flush(&self) -> std::io::Result<()> {
        #[cfg(feature = "log_files")]
        if let Ok(ref mut log_writer) = self.log_writer.lock() {
            if log_writer.is_some() {
                log_writer.as_mut().unwrap().flush()
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }

        #[cfg(not(feature = "log_files"))]
        Ok(())
    }

    /// Loads all settings from a config file.
    ///
    /// It is worth noting that this will overwrite any settings that are already
    /// set, including ones not in the config file.
    ///
    /// # Example
    ///
    /// ```
    /// use pokey_logger::LOGGER;
    ///
    /// LOGGER.load_config_file("/examples/full_usage/config.yml");
    /// ```
    pub fn load_config_file(&self, path: &str) -> Result<(), ConfigFileLoadError> {
        let config_file = ConfigFile::load(path)?;
        self.set_level(config_file.level);
        self.set_color(config_file.color);
        self.set_should_show_time(config_file.time_stamp);
        #[cfg(feature = "log_files")]
        self.set_log_file_color(config_file.file_color);
        #[cfg(feature = "log_files")]
        self.set_existing_log_handler(config_file.existing_log_handler);
        #[cfg(feature = "log_files")]
        if let Some(ref log_path) = config_file.log_file_path {
            self.set_log_path(log_path);
        } else {
            self.remove_log_path();
        }

        debug!("Config file loaded: {:?}", config_file);

        Ok(())
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

/// Set the log level of the global logger.
#[deprecated(since = "0.2.0", note = "Use `set_level` on `LOGGER` instead")]
pub fn set_level(level: Level) {
    LOGGER.set_level(level);
}

/// Set whether or not the global logger should show colors.
#[deprecated(since = "0.2.0", note = "Use `set_color` on `LOGGER` instead")]
pub fn set_color(color: bool) {
    LOGGER.set_color(color);
}
