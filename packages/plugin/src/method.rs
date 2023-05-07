use polywrap_core::{invoker::Invoker, env::Env};

use crate::error::PluginError;

pub type PluginMethod = dyn Fn(&[u8], Option<&Env>, &dyn Invoker) -> Result<Vec<u8>, PluginError> + Send + Sync;