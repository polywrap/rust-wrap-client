use std::{fs, path::Path};

use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    file_reader::FileReader,
    loader::Loader,
    uri::Uri,
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriWrapper},
    uri_resolver::UriResolver,
};
use polywrap_wasm::wasm_wrapper::{WasmWrapper, WasmWrapperConfig};

pub struct FilesystemResolver {
    file_reader: Box<dyn FileReader>,
}

impl FilesystemResolver {
    pub fn new(file_reader: Box<dyn FileReader>) -> Self {
        Self { file_reader }
    }
}

#[async_trait]
impl UriResolver for FilesystemResolver {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: &dyn Loader,
        _: &UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        if uri.authority != "fs" && uri.authority != "file" {
            return Err(Error::ResolutionError("Invalid authority".to_string()));
        };

        let manifest_search_pattern = "wrap.info";
        let manifest_path = Path::new(&uri.path).join(manifest_search_pattern);
        if manifest_path.exists() {
            let manifest_result = self
                .file_reader
                .read_file(&manifest_path.to_str().unwrap())?;

            // let manifest = manifest_result.unwrap();
            let wrapper_path = Path::new(&uri.path).join("wrap.wasm");
            let wrapper_file = fs::read(wrapper_path).unwrap();
            let wrapper_config = WasmWrapperConfig {
                wasm_module: polywrap_wasm::wasm_runtime::instance::WasmModule::Bytes(wrapper_file),
            };
            let wasm_wrapper = WasmWrapper::new(wrapper_config);
            let uri_package_or_wrapper = UriPackageOrWrapper::Package(
                uri.clone(),
                UriWrapper {
                    uri: uri.clone(),
                    wrapper: Box::new(wasm_wrapper),
                },
            );
            return Ok(uri_package_or_wrapper);
        } else {
            return Err(Error::ResolutionError(format!(
                "Failed to find manifest file: {}",
                manifest_search_pattern
            )));
        }
    }
}
