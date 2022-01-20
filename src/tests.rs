use super::*;

#[test]
fn test_get_and_set_colour() {
    let logger = Logger::new();

    logger.set_color(true);
    assert_eq!(logger.get_color(), true);
    logger.set_color(false);
    assert_eq!(logger.get_color(), false);

    LOGGER.set_color(true);
    assert_eq!(LOGGER.get_color(), true);
    LOGGER.set_color(false);
    assert_eq!(LOGGER.get_color(), false);

    set_color(true);
    assert_eq!(LOGGER.get_color(), true);
    set_color(false);
    assert_eq!(LOGGER.get_color(), false);

    // reset to default
    set_color(true);
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

    set_level(Level::Debug);
    assert_eq!(LOGGER.get_level(), Level::Debug);
    set_level(Level::Info);
    assert_eq!(LOGGER.get_level(), Level::Info);
    set_level(Level::Warn);
    assert_eq!(LOGGER.get_level(), Level::Warn);
    set_level(Level::Error);
    assert_eq!(LOGGER.get_level(), Level::Error);

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
        ("NoNe", Level::None)
    ];

    for (input, expected) in expected_pairs {
        assert_eq!(Level::from_str(input), Ok(expected));
    }

    assert!(Level::from_str("").is_err());
    assert!(Level::from_str("foobar").is_err());
}
