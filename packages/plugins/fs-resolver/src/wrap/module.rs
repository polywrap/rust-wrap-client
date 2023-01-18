/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::invoke::Invoker;
use polywrap_plugin::error::PluginError;
use polywrap_plugin::module::PluginModule;
use serde::{Serialize, Deserialize};
use super::types::*;
use async_trait::async_trait;

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
