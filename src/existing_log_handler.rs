//! This module contains the different methods and handlers for what to do
//! when a log file already exists.

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

/// The method of handling a pre-existing log file when starting a new session.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ExistingLogHandler {
    /// Append to the existing log file.
    Append,
    /// Overwrite the existing log file.
    Overwrite,
    /// Rename the existing log file with date and time appended to the name.
    Rename
}

impl Default for ExistingLogHandler {
    fn default() -> Self {
        ExistingLogHandler::Append
    }
}

#[derive(Debug)]
pub enum ExistingLogHandlerOpenError {
    /// Something went wrong when opening the existing log file or creating a new one.
    Io(io::Error),
    /// The path could not be used as a log file. This could because the path
    /// is not a file.
    InvalidPath
}

impl From<io::Error> for ExistingLogHandlerOpenError {
    fn from(error: io::Error) -> Self {
        ExistingLogHandlerOpenError::Io(error)
    }
}

impl ExistingLogHandler {
    pub fn open_file<P: AsRef<Path>>(&self, path: P) -> Result<File, ExistingLogHandlerOpenError> {
        match self {
            ExistingLogHandler::Append => {
                if path.as_ref().exists() {
                    debug!("Appending to existing log file");
                    match File::options().append(true).open(path) {
                        Ok(file) => Ok(file),
                        Err(e) => Err(ExistingLogHandlerOpenError::Io(e))
                    }
                } else {
                    debug!("Creating new log file");
                    match File::create(path) {
                        Ok(file) => Ok(file),
                        Err(e) => Err(ExistingLogHandlerOpenError::Io(e))
                    }
                }
            }
            // TODO: Overwrite should handle new file creation.
            ExistingLogHandler::Overwrite => match File::create(path) {
                Ok(file) => Ok(file),
                Err(e) => Err(ExistingLogHandlerOpenError::Io(e))
            },
            ExistingLogHandler::Rename => {
                let path_buf = PathBuf::from(path.as_ref());
                if path_buf.exists() {
                    // Find the existing extension
                    let existing_extension = match path_buf.extension() {
                        Some(extension) => format!(".{}", extension.to_string_lossy()),
                        None => String::new()
                    };

                    // Add the date before the extension
                    let new_path = path_buf.with_extension(&format!(
                        "{}{}",
                        chrono::Local::now().format("%Y-%m-%d_%H-%M-%S"),
                        existing_extension
                    ));

                    // Rename the file
                    let mut new_file = match File::create(new_path.as_path()) {
                        Ok(file) => file,
                        Err(err) => return Err(ExistingLogHandlerOpenError::Io(err))
                    };

                    // Write the old file to the new file
                    let old_file_content = match std::fs::read_to_string(path_buf.as_path()) {
                        Ok(content) => content,
                        Err(err) => return Err(ExistingLogHandlerOpenError::Io(err))
                    };
                    match new_file.write_all(old_file_content.as_bytes()) {
                        Ok(_) => (),
                        Err(err) => return Err(ExistingLogHandlerOpenError::Io(err))
                    };

                    let res = File::options()
                        .create(true)
                        .write(true)
                        .open(path_buf.as_path());

                    match res {
                        Ok(file) => Ok(file),
                        Err(err) => Err(ExistingLogHandlerOpenError::Io(err))
                    }
                } else {
                    match File::create(path) {
                        Ok(file) => Ok(file),
                        Err(err) => Err(ExistingLogHandlerOpenError::Io(err))
                    }
                }
            }
        }
    }
}
