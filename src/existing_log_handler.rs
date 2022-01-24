use serde::{Deserialize, Serialize};

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