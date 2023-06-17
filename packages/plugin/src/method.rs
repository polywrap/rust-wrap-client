use polywrap_core::invoker::Invoker;

use crate::error::PluginError;

pub type PluginMethod =
    dyn Fn(&[u8], Option<&[u8]>, &dyn Invoker) -> Result<Vec<u8>, PluginError> + Send + Sync;
