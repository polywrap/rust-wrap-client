/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_plugin::*;
use serde::*;
use super::types::*;
use std::result::Result;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsGet {
    pub url: String,
    pub request: Option<Request>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsPost {
    pub url: String,
    pub request: Option<Request>,
}

pub trait Module: PluginModule {
  fn get(&mut self, args: &ArgsGet, invoker: Arc<dyn Invoker>) -> Result<Option<Response>, PluginError>;

  fn post(&mut self, args: &ArgsPost, invoker: Arc<dyn Invoker>) -> Result<Option<Response>, PluginError>;
}
