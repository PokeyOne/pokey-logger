use super::*;
use std::str::FromStr;

#[cfg(feature = "log_files")]
mod file_tests;
#[cfg(feature = "log_files")]
mod time_tests;

#[test]
fn test_that_macro_use_compiles() {
    debug!("this is a test message: {}", 3);
}

#[test]
fn test_get_and_set_colour() {
    let logger = Logger::new();

    logger.set_color(true);
    assert!(logger.get_color());
    logger.set_color(false);
    assert!(!logger.get_color());

    LOGGER.set_color(true);
    assert!(LOGGER.get_color());
    LOGGER.set_color(false);
    assert!(!LOGGER.get_color());

    LOGGER.set_color(true);
}

#[test]
fn test_get_and_set_level() {
    let logger = Logger::new();

    logger.set_level(Level::Debug);
    assert_eq!(logger.get_level(), Level::Debug);
    logger.set_level(Level::Info);
    assert_eq!(logger.get_level(), Level::Info);
    logger.set_level(Level::Warn);
    assert_eq!(logger.get_level(), Level::Warn);
    logger.set_level(Level::Error);
    assert_eq!(logger.get_level(), Level::Error);
    logger.set_level(Level::None);
    assert_eq!(logger.get_level(), Level::None);

    LOGGER.set_level(Level::Debug);
    assert_eq!(LOGGER.get_level(), Level::Debug);
    LOGGER.set_level(Level::Info);
    assert_eq!(LOGGER.get_level(), Level::Info);
    LOGGER.set_level(Level::Warn);
    assert_eq!(LOGGER.get_level(), Level::Warn);
    LOGGER.set_level(Level::Error);
    assert_eq!(LOGGER.get_level(), Level::Error);
    LOGGER.set_level(Level::None);
    assert_eq!(LOGGER.get_level(), Level::None);

    // Reset the level to the default so that other tests actually log
    // DO NOT REMOVE THIS LINE or you will have problems
    LOGGER.set_level(Level::Debug);
}

#[test]
fn test_level_from_string() {
    let expected_pairs = vec![
        ("info", Level::Info),
        ("debug", Level::Debug),
        ("warn", Level::Warn),
        ("error", Level::Error),
        ("none", Level::None),
        ("InFo", Level::Info),
        ("DeBuG", Level::Debug),
        ("WaRn", Level::Warn),
        ("ErRoR", Level::Error),
        ("NoNe", Level::None),
    ];

    for (input, expected) in expected_pairs {
        assert_eq!(Level::from_str(input), Ok(expected));
    }

    assert!(Level::from_str("").is_err());
    assert!(Level::from_str("foobar").is_err());
}
