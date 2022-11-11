use std::sync::Arc;

use polywrap_core::invoke::Invoker;
use serde_json::Value;

pub type PluginMethod = dyn Fn(Value, Arc<dyn Invoker>) -> Result<Value, polywrap_core::error::Error> + Send + Sync;