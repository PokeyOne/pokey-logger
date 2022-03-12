//! This is the module that deals will formatting log messages into different
//! colours and similar for different situations.

#[cfg(test)]
mod tests;

use crate::{Level, TermColor};

/// A message to log. It is basically a wrapper around format calls so that
/// format is only called as needed. For example, if we are logging the colored
/// message to the terminal and the uncolored message to the file, we will have
/// to calculate both. If we are logging the uncolored message to both, then
/// we don't want to calculate the colored message. So this essentially stores
/// the information needed to format the message, and then only formats it when
/// needed.
pub struct LogMessage {
    /// The cached result of the output with colour.
    colorized: Option<String>,
    /// The cached result of the output without colour.
    non_colorized: Option<String>,
    /// The prefix to attach to the message.
    prefix: String,
    /// The name of the level.
    level_string: String,
    /// The colour of the level.
    level_color: TermColor,
    /// The actual message itself.
    message: String
}

impl LogMessage {
    /// Create a new message from the parameters given.
    pub fn new(prefix: &str, message: &str, level: Level) -> LogMessage {
        LogMessage {
            colorized: None,
            non_colorized: None,
            prefix: prefix.to_string(),
            level_string: format!("[{}]", level),
            level_color: level.get_color(),
            message: message.to_string()
        }
    }

    /// Get the output with colour in it.
    ///
    /// This will use the cached value if available.
    pub fn colorized(&mut self) -> String {
        match self.colorized {
            Some(ref s) => s.clone(),
            None => {
                let level_string = self.level_color.colorize(&self.level_string);
                self.colorized = Some(format!(
                    "{}{} {}\n",
                    self.prefix, level_string, self.message
                ));

                self.colorized.clone().unwrap()
            }
        }
    }

    /// Get the output without colour in it.
    ///
    /// This will use the cached value if available.
    pub fn non_colorized(&mut self) -> String {
        match self.non_colorized {
            Some(ref s) => s.clone(),
            None => {
                self.non_colorized = Some(format!(
                    "{}{} {}\n",
                    self.prefix, self.level_string, self.message
                ));

                self.non_colorized.clone().unwrap()
            }
        }
    }

    /// Delegates to either the colorized or non_colorized methods based on
    /// the boolean given.
    pub fn formatted(&mut self, colorized: bool) -> String {
        if colorized {
            self.colorized()
        } else {
            self.non_colorized()
        }
    }
}
