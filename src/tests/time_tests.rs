//! These are all the top-level test that require the 'time' feature.

use super::*;

#[test]
fn test_get_and_set_timestamp_format() {
    let logger = Logger::new();
    let format = "%Y-%m-%d %H:%M:%S".to_string();

    logger.set_timestamp_format(Some(format.clone()));
    assert_eq!(logger.get_timestamp_format(), Some(format));

    logger.info("This is a test of the cool time format");
}
