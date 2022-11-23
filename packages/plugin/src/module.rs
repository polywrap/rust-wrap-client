use std::{sync::Arc};
use async_trait::async_trait;
use serde_json::Value;

use polywrap_core::error::Error;

#[async_trait]
pub trait PluginModule: Send + Sync {
    async fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &Value,
        invoker: Arc<dyn polywrap_core::invoke::Invoker>,
    ) -> Result<Value, Error>;
}
