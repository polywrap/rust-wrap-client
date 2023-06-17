use std::{
    fmt::{Debug, Formatter},
    sync::Arc,
};

use polywrap_core::{
    file_reader::FileReader,
    package::{GetManifestOptions, WrapPackage},
    wrapper::Wrapper,
};
use wrap_manifest_schemas::{
    deserialize::{deserialize_wrap_manifest, DeserializeManifestOptions},
    versions::WrapManifest,
};

use crate::{wasm_module::CompiledWasmModule, wasm_wrapper::WasmWrapper};

use super::file_reader::InMemoryFileReader;

pub struct WasmPackage {
    file_reader: Arc<dyn FileReader>,
    manifest: Option<Vec<u8>>,
    wasm_module: Option<Vec<u8>>,
}

impl WasmPackage {
    pub fn new(
        file_reader: Arc<dyn FileReader>,
        manifest: Option<Vec<u8>>,
        wasm_module: Option<Vec<u8>>,
    ) -> Self {
        Self {
            file_reader: match wasm_module.clone() {
                Some(module) => Arc::new(InMemoryFileReader::new(file_reader, None, Some(module))),
                None => file_reader,
            },
            manifest,
            wasm_module,
        }
    }

    pub fn get_wasm_module(&self) -> Result<Vec<u8>, polywrap_core::error::Error> {
        if self.wasm_module.is_some() {
            return Ok(self.wasm_module.clone().unwrap());
        }

        let file_content = self.file_reader.read_file("wrap.wasm")?;

        Ok(file_content)
    }
}

impl PartialEq for WasmPackage {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Debug for WasmPackage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            r#"
        WasmPackage
        
        -Wasm Module: {:?}
        -Manifest: {:?}
        "#,
            self.wasm_module, self.manifest
        )
    }
}

impl WrapPackage for WasmPackage {
    fn get_manifest(
        &self,
        options: Option<&GetManifestOptions>,
    ) -> Result<WrapManifest, polywrap_core::error::Error> {
        let encoded_manifest = match self.manifest.clone() {
            Some(manifest) => manifest,
            None => self.file_reader.read_file("wrap.info")?,
        };

        let opts = options.map(|options| DeserializeManifestOptions {
            no_validate: options.no_validate,
            ext_schema: None,
        });
        let deserialized_manifest = deserialize_wrap_manifest(&encoded_manifest, opts)
            .map_err(|e| polywrap_core::error::Error::ManifestError(e.to_string()))?;

        Ok(deserialized_manifest)
    }

    fn create_wrapper(&self) -> Result<Arc<dyn Wrapper>, polywrap_core::error::Error> {
        let wasm_module = self.get_wasm_module()?;
        let compiled_module = CompiledWasmModule::from_byte_code(&wasm_module)?;

        Ok(Arc::new(WasmWrapper::new(
            compiled_module,
            self.file_reader.clone(),
        )))
    }
}
