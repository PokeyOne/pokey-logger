//! Convenience functions for working with time and timestamps for logging.

#[cfg(test)]
mod tests;

use chrono::prelude::*;

/// Gets the string to write to the log already formatted to be printed.
///
/// An example might be: "\[18:14:09\]" (using the default format). A custom
/// format can be provided using the same syntax as what chrono offers in its
/// [`chrono::format::strftime`]
///
/// The default format string is "%H:%M:%S"
pub fn current_time_box(format_string: Option<String>) -> String {
    let format_string = match format_string {
        Some(fs) => fs,
        None => "%H:%M:%S".to_string()
    };

    // Get the time
    let time: DateTime<Local> = Local::now();
    // Format the time
    format!("[{}]", time.format(&format_string))
}
