#[macro_use]
extern crate pokey_logger;

use pokey_logger::LOGGER;

fn main() {
    match LOGGER.load_config_file("examples/full_usage/config.yml") {
        Ok(_) => info!("Config file loaded"),
        Err(e) => error!("Error loading config file: {e:?}")
    }
    info!("Hello, world!");
    debug!("This message will be filtered out because of the config file");
}