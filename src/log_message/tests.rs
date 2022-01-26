use super::*;

#[test]
fn test_log_message_new_has_no_cache() {
    let log_message = LogMessage::new("[wow]", "test", Level::Info);
    assert_eq!(log_message.colorized, None);
    assert_eq!(log_message.non_colorized, None);
}

#[test]
fn test_log_message_should_create_cache_and_use_it() {
    let mut log_message = LogMessage::new("[wow]", "test", Level::Info);
    log_message.colorized();
    assert!(log_message.colorized.is_some());
    assert!(log_message.non_colorized.is_none());
    log_message.non_colorized();
    assert!(log_message.colorized.is_some());
    assert!(log_message.non_colorized.is_some());
    assert_ne!(
        log_message.colorized.as_ref().unwrap(),
        log_message.non_colorized.as_ref().unwrap()
    );
}

#[test]
fn test_formatted_should_return_proper_string() {
    let mut log_message = LogMessage::new("[wow]", "test", Level::Info);
    assert_eq!(
        log_message.formatted(false),
        "[wow][INFO] test\n".to_string()
    );
    let expected_colorized = format!("[wow]{} test\n", Level::Info.get_color().colorize("[INFO]"));
    assert_eq!(log_message.formatted(true), expected_colorized);
}
