use std::{fs, path::Path, sync::{Arc, Mutex}, fmt};

use polywrap_core::{
    error::Error,
    file_reader::FileReader,
    loader::Loader,
    uri::Uri,
    resolvers::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    resolvers::uri_resolver::UriResolver,
};
use polywrap_wasm::{
    wasm_package::WasmPackage,
};

pub struct FilesystemResolver {
    file_reader: Arc<dyn FileReader>,
}

impl FilesystemResolver {
    pub fn new(file_reader: Arc<dyn FileReader>) -> Self {
        Self { file_reader }
    }
}

impl UriResolver for FilesystemResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        _: &dyn Loader,
        _: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, Error> {
        if uri.authority != "fs" && uri.authority != "file" {
           return Ok(UriPackageOrWrapper::Uri(uri.clone()));
        };

        let manifest_search_pattern = "wrap.info";
        let manifest_path = Path::new(&uri.path).join(manifest_search_pattern);
        if manifest_path.exists() {
            let manifest = self
                .file_reader
                .read_file(manifest_path.to_str().unwrap())?;

            let wrapper_path = Path::new(&uri.path).join("wrap.wasm");
            let wrapper_file = fs::read(wrapper_path).unwrap();
            let wasm_wrapper = WasmPackage::new(
                self.file_reader.clone(),
                Some(manifest),
                Some(wrapper_file),
            );
            let uri_package_or_wrapper = UriPackageOrWrapper::Package(
                uri.clone(),
                Arc::new(Mutex::new(wasm_wrapper)),
            );
            Ok(uri_package_or_wrapper)
        } else {
            Err(Error::ResolutionError(format!(
                "Failed to find manifest file: {}",
                manifest_search_pattern
            )))
        }
    }
}

impl fmt::Debug for FilesystemResolver {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "FilesystemResolver", )
  }
}
