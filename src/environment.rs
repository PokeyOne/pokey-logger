use std::env;
use super::Logger;

pub fn configure(logger: &Logger) {
    match env::var("PL_COLOR") {
        Ok(val) if val == "true" => logger.set_color(true),
        Ok(_) => logger.set_color(false),
        // Unset, or invalid
        _ => {},
    };
}
