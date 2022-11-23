use crate::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait FileReader: Send + Sync {
    async fn read_file(&self, path: &str) -> Result<Vec<u8>, Error>;
}

pub struct SimpleFileReader {}

impl SimpleFileReader {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl FileReader for SimpleFileReader {
    async fn read_file(&self, path: &str) -> Result<Vec<u8>, Error> {
      std::fs::read(path).map_err(|e| Error::FileReadError(e.to_string()))
    }
}
