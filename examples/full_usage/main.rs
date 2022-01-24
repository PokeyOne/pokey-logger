#[macro_use]
extern crate pokey_logger;

use pokey_logger::LOGGER;

fn main() {
    match LOGGER.load_config_file("examples/full_usage/config.yml") {
        Ok(_) => info!("Config file loaded"),
        Err(e) => error!("Error loading config file: {e:?}")
    }
    debug!("This message will be filtered out because of the config file");
    info!("Hello, world!");
    warn!("This is a warning");
    error!("This is an error");

    // This is important to ensure the log files are fully written before
    // shutting down.
    // TODO: Not sure if the above statement is true.
    LOGGER.flush();
}