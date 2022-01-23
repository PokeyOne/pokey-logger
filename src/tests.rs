use super::*;
use std::str::FromStr;

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

#[test]
fn test_set_log_path() {
    let logger = Logger::new();
    // Setting a path that doesn't exist should not work
    assert!(!logger.set_log_path("this/path/is/not/real/file.log"));
    assert!(logger.get_log_path().is_none());
    assert!(!logger.has_log_writer());

    // Setting a directory that exists should not work
    assert!(!logger.set_log_path("test_log_directory/"));
    assert!(logger.get_log_path().is_none());

    // Ensure the other.log is not actually there still
    if PathBuf::from("test_log_directory/other.log").exists()
        && std::fs::remove_file("test_log_directory/other.log").is_err()
    {
        let err_msg = "this test relies on other.log not existing, but could not delete it. Try deleting it and rerunning this test";
        error!("{}", err_msg);
        panic!("{}", err_msg);
    }

    // Directories exist, but file doesn't, should create the file
    assert!(logger.set_log_path("test_log_directory/other.log"));
    let other_log_path = logger
        .get_log_path()
        .expect("logger log path should be set to other.log");
    assert!(
        other_log_path.exists(),
        "The log file should actually be created"
    );
    logger.info("Log file created: other.log");

    // Both directory and file exist
    assert!(logger.set_log_path("test_log_directory/other.log"));
    logger.info("Test of log file creation ran successfully, if this is showing up in test.log then everything is working smoothly");
    // note it is correct that this assertion is after the info because there
    // is no guarantee that the writer will be created before the info is logged
    assert!(logger.has_log_writer());

    logger.flush().unwrap();
}
