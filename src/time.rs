#[cfg(test)]
mod tests;

use chrono::prelude::*;

/// Gets the string to write to the log already formatted to be printed.
///
/// An example might be: "[18:14:09]"
pub fn current_time_box() -> String {
    let time: DateTime<Local> = Local::now();

    format!("[{}]", time.format("%H:%M:%S"))
}
