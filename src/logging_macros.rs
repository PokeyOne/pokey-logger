#[macro_export]
/// Logs a debug message on the global logger.
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::LOGGER.debug(&format!($($arg)*));
    }
}

#[macro_export]
/// Logs an info message on the global logger.
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::LOGGER.info(&format!($($arg)*));
    }
}

#[macro_export]
/// Logs a warning message on the global logger.
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::LOGGER.warn(&format!($($arg)*));
    }
}

#[macro_export]
/// Logs an error message on the global logger.
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::LOGGER.error(&format!($($arg)*));
    }
}
