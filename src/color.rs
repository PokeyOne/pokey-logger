#[allow(dead_code)]
pub enum TermColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset
}

impl TermColor {
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
}

pub fn colorize(color: TermColor, text: &str) -> String {
    format!("{}{}{}", color.starter_sequence(), text, TermColor::Reset.starter_sequence())
}
