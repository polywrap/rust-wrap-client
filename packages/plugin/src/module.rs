use std::{sync::Arc};
use serde_json::Value;

use polywrap_core::error::Error;

pub trait PluginModule: Send + Sync {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &Value,
        invoker: Arc<dyn polywrap_core::invoke::Invoker>,
    ) -> Result<Value, Error>;
}
