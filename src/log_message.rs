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
    colorized: Option<String>,
    non_colorized: Option<String>,
    prefix: String,
    level_string: String,
    level_color: TermColor,
    message: String
}

impl LogMessage {
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

    pub fn formatted(&mut self, colorized: bool) -> String {
        if colorized {
            self.colorized()
        } else {
            self.non_colorized()
        }
    }
}
