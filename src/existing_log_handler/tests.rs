use super::*;
use tempfile::tempdir;

#[test]
fn test_append() -> Result<(), ExistingLogHandlerOpenError> {
    let dir = tempdir()?;
    assert!(dir.path().exists());

    let log_path = dir.path().join("test.log");
    debug!("log_path: {:?}", log_path);

    let mut file = ExistingLogHandler::Append.open_file(log_path.as_path())?;
    assert!(log_path.exists());
    let mut expected_len = file.write(b"Hello, world!")?;
    drop(file);

    // Re-open the file and write another message
    let mut file = ExistingLogHandler::Append.open_file(log_path.as_path())?;
    expected_len += file.write(b"Goodbye, world!")?;
    assert_ne!(0, expected_len);
    drop(file);

    // Check the file contents
    let actual_len = std::fs::read_to_string(log_path)?.as_bytes().len();
    assert_eq!(expected_len, actual_len);

    Ok(())
}

#[test]
fn test_overwrite() -> Result<(), ExistingLogHandlerOpenError> {
    let dir = tempdir()?;
    assert!(dir.path().exists());

    let log_path = dir.path().join("test.log");
    debug!("log_path: {:?}", log_path);

    let mut file = ExistingLogHandler::Overwrite.open_file(log_path.as_path())?;
    assert!(log_path.exists());
    assert_ne!(0, file.write(b"Hello, world!")?);
    drop(file);

    // Re-open the file and write another message
    let mut file = ExistingLogHandler::Overwrite.open_file(log_path.as_path())?;
    let expected_len = file.write(b"Goodbye, world!")?;
    assert_ne!(0, expected_len);
    drop(file);

    // Check the file contents
    let actual_len = std::fs::read_to_string(log_path)?.as_bytes().len();
    assert_eq!(expected_len, actual_len);

    Ok(())
}

#[test]
fn test_rename() -> Result<(), ExistingLogHandlerOpenError> {
    let dir = tempdir()?;
    assert!(dir.path().exists());

    let log_path = dir.path().join("test.log");
    debug!("log_path: {:?}", log_path);

    let mut file = ExistingLogHandler::Rename.open_file(log_path.as_path())?;
    assert!(log_path.exists());
    assert_ne!(0, file.write(b"Hello, world!")?);
    drop(file);

    // Re-open the file and write another message
    let mut file = ExistingLogHandler::Rename.open_file(log_path.as_path())?;
    let expected_len = file.write(b"Goodbye, world!")?;
    assert_ne!(0, expected_len);
    drop(file);

    // The file should not be overwritten and only have the new message
    let actual_text = std::fs::read_to_string(log_path)?;
    let actual_len = actual_text.as_bytes().len();
    assert_eq!(expected_len, actual_len, "actual_text: {:?}", actual_text);

    // There should be a new file with the original name with a date/time stamp
    let dir_entries: Vec<String> = std::fs::read_dir(dir.path())?
        .map(|entry| {
            entry
                .unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        })
        .collect();
    assert_eq!(2, dir_entries.len());
    debug!("dir_entries: {:?}", dir_entries);

    Ok(())
}
