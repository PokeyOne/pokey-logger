#[cfg(test)]
mod tests;

/// Logs a debug message on the global logger. See [`ldebug!`] for logging to
/// a specific logger.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::LOGGER.debug(&format!($($arg)*))
    }
}

/// Logs a debug message to a specific logger. See [`debug!`] for logging to the
/// global logger.
#[macro_export]
macro_rules! ldebug {
    ($logger:expr, $($arg:tt)*) => {
        $logger.debug(&format!($($arg)*))
    }
}

#[macro_export]
/// Logs an info message on the global logger. See [`linfo!`] for logging to a
/// specific logger.
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::LOGGER.info(&format!($($arg)*))
    }
}

#[macro_export]
/// Logs an info message to a specific logger.
/// See [`info!`] for logging to the global logger.
macro_rules! linfo {
    ($logger:expr, $($arg:tt)*) => {
        $logger.info(&format!($($arg)*))
    }
}

#[macro_export]
/// Logs a warning message on the global logger. See [`lwarn!`] for logging to
/// a specific logger.
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::LOGGER.warn(&format!($($arg)*))
    }
}

#[macro_export]
/// Logs a warning message to a specific logger. See [`warn!`] for logging to
/// the global logger.
macro_rules! lwarn {
    ($logger:expr, $($arg:tt)*) => {
        $logger.warn(&format!($($arg)*))
    }
}

#[macro_export]
/// Logs an error message on the global logger. See [`lerror!`] for logging to
/// a specific logger.
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::LOGGER.error(&format!($($arg)*))
    }
}

#[macro_export]
/// Logs an error message to a specific logger. See [`error!`] for logging to
/// the global logger.
macro_rules! lerror {
    ($logger:expr, $($arg:tt)*) => {
        $logger.error(&format!($($arg)*))
    }
}
