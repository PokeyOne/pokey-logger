# Version 0.3.3 (2022-09-11)

- Removed error print statement form the `set_log_path` function.
- `set_log_path` function now returns a `Result` instead of a `bool`.

# Version 0.3.2 (2022-09-11)

- Added `must_use` annotation to the `Logger` method `set_log_path`.

# Version 0.3.1 (2022-03-12)

- Added: More documentation

# Version 0.3 (2022-03-12)

- Removed: The global `set_level` and `set_color` methods that were previously deprecated

- Added: Default feature for `time` stamps
- Added: Default feature for `log_files`
- Added: Optional feature for `config` files. (This **removes config file loading** by default)
- Added: Environment variable based configuration
- Added: Crate documentation for features and configuration.
