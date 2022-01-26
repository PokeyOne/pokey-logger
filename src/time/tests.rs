use super::*;
use crate::debug;

#[test]
pub fn test_time_box_generation() {
    let result = current_time_box(None);

    debug!("Got time box of {}", result);
    assert_eq!(10, result.len());
    assert_eq!(':', result.chars().nth(3).unwrap());
}

#[test]
pub fn test_time_box_generation_with_same_format_string() {
    let format_string = "%H:%M:%S".to_string();
    let std_result = current_time_box(None);
    let custom_result = current_time_box(Some(format_string));

    assert_eq!(std_result, custom_result);
}
