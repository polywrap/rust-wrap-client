use std::{sync::{Arc, Mutex}, fmt::{Formatter,Debug}};
use wrap_manifest_schemas::{
    versions::WrapManifest,
};
use polywrap_core::{error::Error, package::{GetManifestOptions, WrapPackage}, wrapper::Wrapper};

use crate::{module::PluginModule, wrapper::PluginWrapper};

pub struct PluginPackage {
    manifest: WrapManifest,
    plugin_module: Arc<Mutex<Box<dyn PluginModule>>>,
}

impl PluginPackage {
    pub fn new(
        plugin_module: Arc<Mutex<Box<dyn PluginModule>>>,
        manifest: WrapManifest
    ) -> Self {
        Self {
            plugin_module,
            manifest,
        }
    }
}

impl PartialEq for PluginPackage {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Debug for PluginPackage {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
      write!(f, r#"
      Plugin Package
      
      -Plugin Module: {:?}
      -Manifest: {:?}
      "#, self.plugin_module, self.manifest)
    }
}

impl WrapPackage for PluginPackage {
    fn get_manifest(
        &self,
        _: Option<GetManifestOptions>,
    ) -> Result<WrapManifest, Error> {
        Ok(self.manifest.clone())
    }

    fn create_wrapper(&self) -> Result<Arc<Mutex<Box<dyn Wrapper>>>, Error> {
        Ok(Arc::new(Mutex::new(Box::new(PluginWrapper::new(self.plugin_module.clone())))))
    }
}
