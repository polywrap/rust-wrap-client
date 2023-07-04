use polywrap_core::{
    error::Error,
    package::{GetManifestOptions, WrapPackage},
    wrapper::Wrapper,
};
use std::{
    fmt::{Debug, Formatter},
    sync::{Arc, Mutex},
};
use wrap_manifest_schemas::versions::WrapManifest;

use crate::{module::PluginModule, wrapper::PluginWrapper};

pub struct PluginPackage<T: PluginModule> {
    manifest: WrapManifest,
    plugin_module: Arc<Mutex<T>>,
}

impl<T: PluginModule> PluginPackage<T> {
    pub fn new(plugin_module: Arc<Mutex<T>>, manifest: WrapManifest) -> Self {
        Self {
            plugin_module,
            manifest,
        }
    }
}

impl<T: PluginModule> PartialEq for PluginPackage<T> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl<T: PluginModule> Debug for PluginPackage<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            r#"
      Plugin Package
      
      -Plugin Module: {:?}
      -Manifest: {:?}
      "#,
            self.plugin_module, self.manifest
        )
    }
}

impl<T: PluginModule + 'static> WrapPackage for PluginPackage<T> {
    fn get_manifest(&self, _: Option<&GetManifestOptions>) -> Result<WrapManifest, Error> {
        Ok(self.manifest.clone())
    }

    fn create_wrapper(&self) -> Result<Arc<dyn Wrapper>, Error> {
        Ok(Arc::new(PluginWrapper::new(self.plugin_module.clone())))
    }
}
