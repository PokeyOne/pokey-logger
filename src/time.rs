//! Convenience functions for working with time and timestamps for logging.

#[cfg(test)]
mod tests;

use chrono::prelude::*;

/// Gets the string to write to the log already formatted to be printed.
///
/// An example might be: "[18:14:09]"
pub fn current_time_box() -> String {
    // Get the time
    let time: DateTime<Local> = Local::now();
    // Format the time
    format!("[{}]", time.format("%H:%M:%S"))
}
