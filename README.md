# Pokey Logger

![gpl 3 badge](https://img.shields.io/badge/license-GPL%203.0-blue)
[![crates.io badge](https://img.shields.io/crates/v/pokey_logger)](https://crates.io/crates/pokey_logger)
[![crates.io downloads](https://img.shields.io/crates/d/pokey_logger)](https://crates.io/crates/pokey_logger)

A simple logging library for Rust.

## Usage

First, the library must be added to the project's `Cargo.toml` file.
```toml
pokey_logger = "0.3.3"
```
or to get the latest and greatest
```toml
pokey_logger = { git = "https://github.com/PokeyOne/pokey-logger" }
```
For more advanced methods see [the Cargo documentation on specifiying dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)

### Usage in Rust

This section of the README will guide you through the basic steps of getting
started with the logger.

In your main rust file, you will want something like this:
```rust
#[macro_use]
extern crate pokey_logger;
```

This will allow you to use the macros from this crate. In your code, you can
now access all the macros.

```rust
fn main() {
    debug!("Hello");
    info!("Hàlo, fàilte!");
    warn!("This program will end soon...");
    error!("No more code");
}
```

And this will output to the terminal, and all is well. You can then take a look
at the crate documentation which goes more in depth on configuration,
file-based logging, and more.

## Documentation
To view the documentation locally, run the following command either in this
repo or in a project that uses the library.
```bash
cargo doc --open
```
Or go to the [crate documentation](https://docs.rs/pokey_logger/latest/pokey_logger/)
