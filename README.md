# Pokey Logger

![gpl 3 badge](https://img.shields.io/badge/license-GPL%203.0-blue)

A simple logging library for Rust.

## Usage

First, the library must be added to the project's `Cargo.toml` file.
```toml
pokey_logger = "0.2.0"
```
or to get the latest and greatest
```toml
pokey_logger = { git = "https://github.com/PokeyOne/pokey-logger" }
```
For more advanced methods see [the Cargo documentation on specifiying dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)

### Usage in Rust

For complete instructions, see the rustdoc documentation. Below is a simple
example.

This is an example of logging some messages. It is assumed that before this
that the `debug!` macro, the `Level` type, and the `LOGGER` constant have been
imported.
```rust
use pokey_logger::{Level, LOGGER, debug, warn};

fn main() {
    // Optionally you can configure the output log level and whether or not colours
    // are shown in the terminal
    LOGGER.set_color(true);
    LOGGER.set_level(Level::Debug);
    if !LOGGER.set_log_path("logs/server.log") {
        warn!("Could not set log path");
    }

    // This will print a debug message using the `debug!` macro. The available macros
    // are debug, info, warn, and error.
    // The usage is exactly the same as a format! or println! macro.
    debug!("Some message with the number {} in it", 4);
}
```

As of version 0.2.0 of the library a log file can be added. It should be noted
that the library will **never** create directories, but it will create log files
if they don't exist. For example in the above example program, the logger would
not create the logs directory, but if the logs directory existed and the file
did not, it would be able to create the `server.log` file.

It is also valuable to note that `LOGGER` is a global static instance of the
`Logger`. It is thread safe to use, but one should be careful about configuring
its settings from multiple threads. If you would like separate configurations
and instances, the `Logger` struct itself can be instantiated and passed around
as the developer sees fit.
