/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::{invoke::Invoker};
use polywrap_plugin::error::PluginError;
use polywrap_plugin::module::PluginModule;
use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsRequiredEnv {
    pub arg: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsOptEnv {
    pub arg: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsNoEnv {
    pub arg: Option<String>,
}

pub trait Module: PluginModule {
  fn required_env(&mut self, args: &ArgsRequiredEnv, invoker: Arc<dyn Invoker>, env: Env) -> Result<Option<bool>, PluginError>;

  fn opt_env(&mut self, args: &ArgsOptEnv, invoker: Arc<dyn Invoker>, env: Option<Env>) -> Result<Option<bool>, PluginError>;

  fn no_env(&mut self, args: &ArgsNoEnv, invoker: Arc<dyn Invoker>) -> Result<Option<bool>, PluginError>;
}
