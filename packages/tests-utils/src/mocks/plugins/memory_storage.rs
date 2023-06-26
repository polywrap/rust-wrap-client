use polywrap_core::invoker::Invoker;
use polywrap_plugin::error::PluginError;
use polywrap_plugin::{implementor::plugin_impl, module::PluginModule, JSON};
use serde::{Deserialize, Serialize};

use std::{fmt::Debug, sync::Arc, thread::sleep, time::Duration};
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetData {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsSetData {
    pub value: i32,
}

pub trait Module: PluginModule {
    fn get_data(
        &mut self,
        args: &ArgsGetData,
        invoker: Arc<dyn Invoker>,
    ) -> Result<i32, PluginError>;

    fn set_data(
        &mut self,
        args: &ArgsSetData,
        invoker: Arc<dyn Invoker>,
    ) -> Result<bool, PluginError>;
}

#[derive(Debug)]
pub struct MemoryStoragePlugin {
    pub value: i32,
}

#[plugin_impl]
impl Module for MemoryStoragePlugin {
    fn get_data(
        &mut self,
        _args: &ArgsGetData,
        _invoker: Arc<dyn Invoker>,
    ) -> Result<i32, PluginError> {
        sleep(Duration::from_millis(50));
        Ok(self.value)
    }

    fn set_data(
        &mut self,
        args: &ArgsSetData,
        _invoker: Arc<dyn Invoker>,
    ) -> Result<bool, PluginError> {
        sleep(Duration::from_millis(50));
        self.value = args.value;
        Ok(true)
    }
}

pub fn get_manifest() -> WrapManifest {
    WrapManifest {
        name: "MemoryStorage".to_string(),
        type_: "plugin".to_string(),
        version: "0.1".to_string(),
        abi: JSON::from_value::<WrapManifestAbi>(JSON::json!({
          "moduleType": {},
          "version": "0.1"
        }))
        .unwrap(),
    }
}
