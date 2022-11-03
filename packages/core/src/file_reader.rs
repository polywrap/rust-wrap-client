use crate::error::Error;

pub trait FileReader: Send + Sync {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, Error>;
}

pub struct SimpleFileReader {}

impl SimpleFileReader {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileReader for SimpleFileReader {
    fn read_file(&self, path: &str) -> Result<Vec<u8>, Error> {
        let result = std::fs::read(path);
        if result.is_err() {
            return Err(Error::GetFileError(format!(
                "Failed to read file: {}",
                result.err().unwrap()
            )));
        } else {
            return Ok(result.unwrap());
        }
    }
}
