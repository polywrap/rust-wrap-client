use async_trait::async_trait;
use polywrap_core::{file_reader::FileReader, manifest::Manifest, package::{WrapPackage, GetManifestOptions}};

use super::file_reader::InMemoryFileReader;

pub struct WasmPackage {
    file_reader: Box<dyn FileReader>,
    manifest: Option<Manifest>,
    wasm_module: Option<Vec<u8>>,
}

impl WasmPackage {
    pub fn new(
        file_reader: Box<dyn FileReader>,
        manifest: Option<Manifest>,
        wasm_module: Option<Vec<u8>>,
    ) -> Self {
        Self {
            file_reader: match wasm_module {
                Some(module) => Box::new(InMemoryFileReader::new(file_reader, None, Some(module))),
                None => file_reader,
            },
            manifest,
            wasm_module,
        }
    }

    pub async fn get_wasm_module(&self) -> Result<Vec<u8>, polywrap_core::error::Error> {
        if self.wasm_module.is_some() {
            return Ok(self.wasm_module.clone().unwrap());
        }

        let file_content = self.file_reader.read_file("wrap.wasm")?;

        Ok(file_content)
    }
}


#[async_trait]
impl WrapPackage for WasmPackage {
  async fn get_manifest(&self, options: Option<GetManifestOptions>) -> Result<Manifest, polywrap_core::error::Error> {
    if self.manifest.is_some() {
      return Ok(self.manifest.clone().unwrap());
    }

    let file_content = self.file_reader.read_file("wrap.info")?;

    let manifest: Manifest = rmp_serde::from_slice(file_content.as_slice())
      .map_err(|e| polywrap_core::error::Error::WasmWrapperError(e.to_string()))?;

    Ok(manifest)
  }
}
