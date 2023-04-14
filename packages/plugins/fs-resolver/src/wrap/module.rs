/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::{invoke::Invoker, env::Env};
use polywrap_plugin::error::PluginError;
use polywrap_plugin::module::PluginModule;
use serde::{Serialize, Deserialize};
use super::types::*;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsTryResolveUri {
    pub authority: String,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGetFile {
    pub path: String,
}

pub trait Module: PluginModule {
  fn try_resolve_uri(&mut self, args: &ArgsTryResolveUri, env: Option<Env>, invoker: Arc<dyn Invoker>) -> Result<Option<MaybeUriOrManifest>, PluginError>;

  fn get_file(&mut self, args: &ArgsGetFile, env: Option<Env>, invoker: Arc<dyn Invoker>) -> Result<Option<Vec<u8>>, PluginError>;
}
