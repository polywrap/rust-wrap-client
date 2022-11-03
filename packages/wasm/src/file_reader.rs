use polywrap_core::file_reader::FileReader;

pub struct BaseFileReader {}

impl FileReader for BaseFileReader {
    fn read_file(&self, file_path: &str) -> Result<Vec<u8>, polywrap_core::error::Error> {
        let contents = std::fs::read(file_path)
            .map_err(|e| polywrap_core::error::Error::WasmWrapperError(e.to_string()))?;
        Ok(contents)
    }
}

pub struct InMemoryFileReader {
    wasm_manifest: Option<Vec<u8>>,
    wasm_module: Option<Vec<u8>>,
    base_file_reader: Box<dyn FileReader>,
}

impl InMemoryFileReader {
    pub fn new(
        base_file_reader: Box<dyn FileReader>,
        wasm_manifest: Option<Vec<u8>>,
        wasm_module: Option<Vec<u8>>,
    ) -> Self {
        Self {
            wasm_manifest,
            wasm_module,
            base_file_reader,
        }
    }
}

impl FileReader for InMemoryFileReader {
    fn read_file(&self, file_path: &str) -> Result<Vec<u8>, polywrap_core::error::Error> {
        if file_path == "wrap.wasm" && self.wasm_module.is_some() {
            return Ok(self.wasm_module.clone().unwrap());
        } else if file_path == "wrap.info" && self.wasm_manifest.is_some() {
            return Ok(self.wasm_manifest.clone().unwrap());
        } else {
            return self.base_file_reader.read_file(file_path);
        }
    }
}
