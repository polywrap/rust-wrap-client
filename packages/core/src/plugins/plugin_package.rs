use std::sync::Arc;

use async_trait::async_trait;
use polywrap_manifest::{
    deserialize::{deserialize_polywrap_manifest, DeserializeManifestOptions},
    versions::WrapManifest,
};

use crate::{
    error::Error,
    file_reader::FileReader,
    package::{GetManifestOptions, WrapPackage},
    wrapper::Wrapper,
};

use super::{plugin_module::PluginModule, plugin_wrapper::PluginWrapper};

pub struct PluginPackage {
    file_reader: Arc<dyn FileReader>,
    manifest: Option<Vec<u8>>,
    plugin_module: Arc<dyn PluginModule>,
}

impl PluginPackage {
    pub fn new(
        plugin_module: Arc<dyn PluginModule>,
        manifest: Vec<u8>,
        file_reader: Arc<dyn FileReader>,
    ) -> Self {
        Self {
            plugin_module,
            file_reader,
            manifest: Some(manifest),
        }
    }
}

#[async_trait]
impl WrapPackage for PluginPackage {
    async fn get_manifest(
        &self,
        options: Option<GetManifestOptions>,
    ) -> Result<WrapManifest, Error> {
        let encoded_manifest = match self.manifest.clone() {
            Some(manifest) => manifest,
            None => self.file_reader.read_file("wrap.info")?,
        };

        let opts = options.map(|options| DeserializeManifestOptions {
            no_validate: options.no_validate,
            ext_schema: None,
        });
        let deserialized_manifest = deserialize_polywrap_manifest(&encoded_manifest, opts)
            .map_err(|e| Error::ManifestError(e.to_string()))?;

        return Ok(deserialized_manifest);
    }

    async fn create_wrapper(&self) -> Result<Box<dyn Wrapper>, Error> {
        Ok(Box::new(PluginWrapper::new(self.plugin_module.clone())))
    }
}
