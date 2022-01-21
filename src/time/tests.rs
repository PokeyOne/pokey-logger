use super::*;
use crate::debug;

#[test]
pub fn test_time_box_generation() {
    let result = current_time_box();

    debug!("Got time box of {}", result);
    assert_eq!(10, result.len());
}
