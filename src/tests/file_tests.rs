//! These are all the top-level tests that require the 'log_files' feature.

use super::*;

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
