use std::sync::Arc;

use polywrap_core::invoke::Invoker;

use crate::error::PluginError;

pub type PluginMethod = dyn Fn(&[u8], Arc<dyn Invoker>) -> Result<Vec<u8>, PluginError> + Send + Sync;