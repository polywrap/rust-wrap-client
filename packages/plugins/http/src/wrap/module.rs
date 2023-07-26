/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::invoker::Invoker;
use polywrap_plugin::{error::PluginError, module::PluginModule};
use polywrap_msgpack_serde::{
  to_vec,
  from_slice,
  BigInt,
  BigNumber,
  JSON,
  bytes,
  wrappers::{
    polywrap_bigint as bigint,
    polywrap_json as json
  },
  JSONString
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use super::types::*;

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
