/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_plugin::*;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsLog {
    pub level: LogLevel,
    pub message: String,
}

pub trait Module: PluginModule {
  fn log(&mut self, args: &ArgsLog, invoker: Arc<dyn Invoker>) -> Result<bool, PluginError>;
}
