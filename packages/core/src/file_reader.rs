use crate::{error::Error, client::Client};


pub trait FileReader: Send + Sync {
    fn read_file(&self, client: &dyn Client, path: &str) -> Result<Vec<u8>, Error>;
}

pub struct SimpleFileReader {}

impl Default for SimpleFileReader {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleFileReader {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileReader for SimpleFileReader {
    fn read_file(&self, _: &dyn Client, path: &str) -> Result<Vec<u8>, Error> {
      std::fs::read(path).map_err(|e| Error::FileReadError(e.to_string()))
    }
}
