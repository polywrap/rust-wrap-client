use std::fmt::Debug;
use crate::error::Error;

pub trait FileReader: Debug + Send + Sync {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, Error>;
}

#[derive(Debug)]
pub struct SimpleFileReader {}

impl SimpleFileReader {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileReader for SimpleFileReader {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, Error> {
      std::fs::read(path).map_err(|e| Error::FileReadError(e.to_string()))
    }
}
