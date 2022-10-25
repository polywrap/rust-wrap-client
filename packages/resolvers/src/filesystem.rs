use std::{fs, path::Path};

use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    loader::Loader,
    uri::Uri,
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext, UriWrapper},
    uri_resolver::UriResolver,
};
use polywrap_wasm::wasm_wrapper::{WasmWrapper, WasmWrapperConfig};

pub struct FilesystemResolver {}

#[async_trait]
impl UriResolver for FilesystemResolver {
    async fn try_resolve_uri(
        &self,
        uri: &Uri,
        loader: &dyn Loader,
        resolution_context: &UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        if uri.authority != "fs" && uri.authority != "file" {
            return Err(Error::ResolutionError("Invalid authority".to_string()));
        };

        let manifest_search_pattern = "wrap.info";
        let manifest_path = Path::new(&uri.path).join(manifest_search_pattern);
        if manifest_path.exists() {
            let manifest_result = fs::read(manifest_path);

            if manifest_result.is_err() {
                return Err(Error::ResolutionError(format!(
                    "Failed to read manifest file: {}",
                    manifest_result.err().unwrap()
                )));
            } else {
                // let manifest = manifest_result.unwrap();
                let wrapper_path = Path::new(&uri.path).join("wrap.wasm");
                let wrapper_file = fs::read(wrapper_path).unwrap();
                let wrapper_config = WasmWrapperConfig {
                    wasm_module: polywrap_wasm::wasm_runtime::instance::WasmModule::Bytes(wrapper_file),
                };
                let wasm_wrapper = WasmWrapper::new(wrapper_config);
                let uri_package_or_wrapper = UriPackageOrWrapper::Wrapper(uri.clone(), UriWrapper {
                  uri: uri.clone(),
                  wrapper: Box::new(wasm_wrapper)
                });
                return Ok(uri_package_or_wrapper);
            }
        } else {
            return Err(Error::ResolutionError(format!(
                "Failed to find manifest file: {}",
                manifest_search_pattern
            )));
        }
    }
}
