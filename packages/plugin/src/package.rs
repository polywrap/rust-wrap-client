use std::{sync::Arc};

use async_trait::async_trait;
use polywrap_manifest::{
    versions::WrapManifest,
};
use polywrap_core::{error::Error, package::{GetManifestOptions, WrapPackage}, wrapper::Wrapper};

use crate::{module::PluginModule, wrapper::PluginWrapper};

pub struct PluginPackage {
    manifest: WrapManifest,
    plugin_module: Arc<dyn PluginModule>,
}

impl PluginPackage {
    pub fn new(
        plugin_module: Arc<dyn PluginModule>,
        manifest: WrapManifest
    ) -> Self {
        Self {
            plugin_module,
            manifest,
        }
    }
}

#[async_trait]
impl WrapPackage for PluginPackage {
    async fn get_manifest(
        &self,
        _: Option<GetManifestOptions>,
    ) -> Result<WrapManifest, Error> {
        return Ok(self.manifest.clone());
    }

    async fn create_wrapper(&self) -> Result<Box<dyn Wrapper>, Error> {
        Ok(Box::new(PluginWrapper::new(self.plugin_module.clone())))
    }
}
