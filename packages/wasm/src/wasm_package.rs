use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex},
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

use crate::{wasm_module::{CompiledWasmModule, WasmModule}, wasm_wrapper::WasmWrapper, error::WrapperError};

use super::file_reader::InMemoryFileReader;

pub struct WasmPackage {
    file_reader: Arc<dyn FileReader>,
    manifest: Option<Vec<u8>>,
    wasm_module: Arc<Mutex<Option<WasmModule>>>,
}

impl WasmPackage {
    pub fn from_byte_code(
        wasm_bytes: Vec<u8>,
        file_reader: Arc<dyn FileReader>,
        manifest: Option<Vec<u8>>,
    ) -> Self {
        Self {
            file_reader: Arc::new(InMemoryFileReader::new(file_reader, None, Some(wasm_bytes.clone()))),
            manifest,
            wasm_module: Arc::new(Mutex::new(Some(WasmModule::WasmByteCode(wasm_bytes)))),
        }
    }

    pub fn from_file_reader(
        file_reader: Arc<dyn FileReader>,
        manifest: Option<Vec<u8>>,
    ) -> Self {
        Self {
            file_reader,
            manifest,
            wasm_module: Arc::new(Mutex::new(None)),
        }
    }

    pub fn from_compiled_module(
        wasm_module: CompiledWasmModule,
        wasm_bytes: Vec<u8>,
        file_reader: Arc<dyn FileReader>,
        manifest: Option<Vec<u8>>,
    ) -> Result<Self, WrapperError> {     
        Ok(Self {
            file_reader: Arc::new(InMemoryFileReader::new(file_reader, None, Some(wasm_bytes))),
            manifest,
            wasm_module: Arc::new(Mutex::new(Some(WasmModule::Compiled(wasm_module)))),
        })
    }

    pub fn get_wasm_module(&self) -> Result<Vec<u8>, polywrap_core::error::Error> {
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
        
        -Manifest: {:?}
        "#,
        self.manifest
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
        let wasm_bytes = self.get_wasm_module()?;

        let mut o_wasm_module = self.wasm_module.lock().unwrap();
        let wasm_module = o_wasm_module.clone();

        if wasm_module.is_some() {
            let compiled_module = wasm_module.unwrap().compile()?;
            *o_wasm_module = Some(WasmModule::Compiled(compiled_module.clone()));

            return Ok(Arc::new(WasmWrapper::new(
                compiled_module.clone(),
                self.file_reader.clone(),
            )));
        } else {
            let compiled_module = CompiledWasmModule::from_byte_code(&wasm_bytes)?;
            *o_wasm_module = Some(WasmModule::Compiled(compiled_module.clone()));

            return Ok(Arc::new(WasmWrapper::new(
                compiled_module.clone(),
                self.file_reader.clone(),
            )));
        }
    }
}
