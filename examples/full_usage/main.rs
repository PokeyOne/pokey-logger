#[macro_use]
extern crate pokey_logger;

use pokey_logger::existing_log_handler::ExistingLogHandler;
use pokey_logger::{Logger, LOGGER};

fn main() {
    // Load a configuration file if the config feature is enabled.
    #[cfg(feature = "config")]
    match LOGGER.load_config_file("examples/full_usage/config.yml") {
        Ok(_) => info!("Config file loaded"),
        Err(e) => error!("Error loading config file: {e:?}")
    }

    // These are all the different log levels
    debug!("This message will be filtered out because of the config file");
    info!("Hello, world!");
    warn!("This is a warning");
    error!("This is an error");

    // This is an example of creating a separate logger instance that also
    // saves a new log every
    file_renaming();

    // This is important to ensure the log files are fully written before
    // shutting down.
    if let Err(e) = LOGGER.flush() {
        error!("Error flushing logs: {e:?}");
    }
}

/// An example of renaming the log file if it already exists before running.
/// Each time it is run, the log file will be renamed with the datestamp, and
/// then the actual log file will be overwritten.
fn file_renaming() {
    let logger = Logger::new();
    logger.set_log_path("examples/full_usage/logs/rename_log.log");
    logger.set_existing_log_handler(ExistingLogHandler::Rename);
    logger.info("This is cool");
    logger.flush().unwrap();
    drop(logger);
}
