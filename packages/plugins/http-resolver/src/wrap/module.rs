/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::invoke::Invoker;
use polywrap_plugin::error::PluginError;
use polywrap_plugin::module::PluginModule;
use serde::{Serialize, Deserialize};
use super::types::*;
use async_trait::async_trait;
pub use polywrap_plugin::impl_plugin_traits;

#[macro_export]
macro_rules! impl_traits {
    ($plugin_type:ty) => {
        $crate::wrap::module::impl_plugin_traits!(
            $plugin_type,
            $crate::wrap::wrap_info::get_manifest(),
            (try_resolve_uri, $crate::wrap::module::ArgsTryResolveUri),
            (get_file, $crate::wrap::module::ArgsGetFile),
        );
    };
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsTryResolveUri {
    pub authority: String,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetFile {
    pub path: String,
}

#[async_trait]
pub trait Module: PluginModule {
  async fn try_resolve_uri(&mut self, args: &ArgsTryResolveUri, invoker: Arc<dyn Invoker>) -> Result<Option<MaybeUriOrManifest>, PluginError>;

  async fn get_file(&mut self, args: &ArgsGetFile, invoker: Arc<dyn Invoker>) -> Result<Option<Vec<u8>>, PluginError>;
}
