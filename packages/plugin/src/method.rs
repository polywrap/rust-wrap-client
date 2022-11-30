use std::sync::Arc;

use polywrap_core::invoke::Invoker;
use serde_json::Value;

use crate::error::PluginError;

pub type PluginMethod = dyn Fn(Value, Arc<dyn Invoker>) -> Result<Value, PluginError> + Send + Sync;