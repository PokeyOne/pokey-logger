//! All things to do with colours and their output to the terminal.

/// An output colour for the terminal. This is a wrapper around the ANSI
/// colour codes.
#[allow(dead_code)]
pub enum TermColor {
    /// Black colour. ANSI code = 30
    Black,
    /// Red colour. ANSI code = 31
    Red,
    /// Green colour. ANSI code = 32
    Green,
    /// Yellow colour. ANSI code = 33
    Yellow,
    /// Blue colour. ANSI code = 34
    Blue,
    /// Magenta colour. ANSI code = 35
    Magenta,
    /// Cyan colour. ANSI code = 36
    Cyan,
    /// White colour. ANSI code = 37
    White,
    /// No colour. ANSI code = 0
    Reset
}

impl TermColor {
    /// Returns the ANSI code for the colour to be used in the string to
    /// indicate the colour.
    pub fn starter_sequence(&self) -> &str {
        match self {
            TermColor::Black => "\x1b[30m",
            TermColor::Red => "\x1b[31m",
            TermColor::Green => "\x1b[32m",
            TermColor::Yellow => "\x1b[33m",
            TermColor::Blue => "\x1b[34m",
            TermColor::Magenta => "\x1b[35m",
            TermColor::Cyan => "\x1b[36m",
            TermColor::White => "\x1b[37m",
            TermColor::Reset => "\x1b[0m"
        }
    }

    /// Adds the colour to the string. Similar to [`colorize`](fn.colorize.html).
    pub fn colorize(&self, string: &str) -> String {
        format!(
            "{}{}{}",
            self.starter_sequence(),
            string,
            TermColor::Reset.starter_sequence()
        )
    }
}

/// Adds the colour code to the start of the string and the reset code to the
/// end. Convenience function for [`TermColor::colorize`](enum.TermColor.html#method.colorize).
pub fn colorize(color: TermColor, text: &str) -> String {
    color.colorize(text)
}
