use std::io;

pub struct ConfigFile {}

#[derive(Debug)]
pub enum ConfigFileLoadError {
    IoError(io::Error)
    // TODO: add serde error
}

impl ConfigFile {
    pub fn load(from_path: &str) -> Result<Self, ConfigFileLoadError> {
        todo!()
    }
}