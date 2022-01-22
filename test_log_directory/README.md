# Test Log Directory

This directory is used for tests to create temporary log files for testing.
No files in this directory should be added to git other than this readme to
document this expectation.

## List of Tests That Use This Directory
This list may or may not be up to date, but as of 2022-01-21 it is. Files used
in here should be logged and tests should not conflict in file usage to avoid
race conditions.

| test | use |
|--|--|
| `tests::test_set_log_path` | removes `other.log` if exists, then creates it again. Leaves it behind on purpose to be easier to check manually |
