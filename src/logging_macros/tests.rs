use crate::{debug, ldebug, Logger};

#[test]
fn test_regular_macro() {
    debug!("test_regular_macro");
}

#[test]
fn test_macro_with_logger() {
    let logger = Logger::new();
    ldebug!(logger, "test_macro_with_logger");
}