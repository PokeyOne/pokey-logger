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
//! For more advanced methods see [the Cargo documentation on specifiying dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
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

#![allow(dead_code)]
// Allow needless doctest main function because example above makes more sense
// with the main function.
#![allow(clippy::needless_doctest_main)]

#[cfg(test)]
mod tests;

mod color;
mod level;
mod time;
#[macro_use]
pub mod logging_macros;

use color::{colorize, TermColor};

use lazy_static::lazy_static;
pub use level::Level;

use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::PathBuf;

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
/// use pokey_logger::{Level, LOGGER, warn};
/// // Only log debug and above. default is info
/// LOGGER.set_level(Level::Debug);
/// // Turn off colors. default is true
/// LOGGER.set_color(false);
/// // Turn off the timestamp. default is true
/// LOGGER.set_should_show_time(false);
/// // Set the log path. default is none. See remove_log_path
/// if !LOGGER.set_log_path("logs/server.log") {
///     warn!("Could not set log path");
/// }
pub struct Logger {
    level: Mutex<Level>,
    color: AtomicBool,
    show_time: AtomicBool,
    log_path: Mutex<Option<PathBuf>>,
    log_writer: Mutex<Option<BufWriter<File>>>
}

impl Logger {
    /// Create a new Logger instance with all default settings. This never
    /// needs to be mutable because all settings are wrapped in an atomic
    /// or mutex reference.
    fn new() -> Logger {
        Logger {
            level: Mutex::new(Level::Debug),
            color: AtomicBool::new(true),
            show_time: AtomicBool::new(true),
            log_path: Mutex::new(None),
            log_writer: Mutex::new(None)
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
    pub fn remove_log_path(&self) {
        *self.log_path.lock().unwrap() = None;
        self.remove_log_writer();
    }

    /// Get the path to the file that the logger is logging to.
    pub fn get_log_path(&self) -> Option<PathBuf> {
        (*self.log_path.lock().unwrap()).as_ref().cloned()
    }

    /// Set the file writer to write actual data to. This method should only
    /// be called internally.
    fn set_log_writer(&self, buf_writer: BufWriter<File>) {
        *self.log_writer.lock().unwrap() = Some(buf_writer);
    }

    /// Remove the file writer. This method should only be called internally.
    fn remove_log_writer(&self) {
        *self.log_writer.lock().unwrap() = None;
    }

    /// Whether or not the logger writer has already been set. If this method
    /// is false but log path is set, a log writer should be created.
    /// See [`set_log_writer_if_not_set`](#method.set_log_writer_if_not_set)
    fn has_log_writer(&self) -> bool {
        if let Ok(lw) = self.log_writer.lock() {
            return lw.is_some();
        }

        false
    }

    /// If there is a log path set, but no log writer, create a log writer.
    fn set_log_writer_if_not_set(&self) {
        if !self.has_log_writer() {
            if let Some(path) = self.get_log_path() {
                let file = File::create(&path).unwrap();
                let buf_writer = BufWriter::new(file);
                self.set_log_writer(buf_writer);
            }
        }
    }

    /// Actually write the log message to the file and stdout. Should only be
    /// called internally by the `debug`, `info`, `warn`, and `error` methods.
    fn log_message(&self, level: Level, message: &str) {
        // Apply colouring to the level indicator
        let level = if self.get_color() {
            colorize(level.get_color(), &format!("[{level}]"))
        } else {
            format!("[{level}]")
        };

        // Format the final string
        let formatted_message = format!("{}{level} {message}\n", self.prefix());

        // Print to stdout
        print!("{}", formatted_message);

        // Write to file
        self.set_log_writer_if_not_set();
        if let Ok(ref mut log_writer) = self.log_writer.lock() {
            if log_writer.is_some() {
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
    fn prefix(&self) -> String {
        if self.should_show_time() {
            time::current_time_box()
        } else {
            "".to_string()
        }
    }

    /// Ensure all io buffers are cleared; usually before shutdown.
    pub fn flush(&self) -> std::io::Result<()> {
        if let Ok(ref mut log_writer) = self.log_writer.lock() {
            if log_writer.is_some() {
                log_writer.as_mut().unwrap().flush()
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
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
