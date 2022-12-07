use std::{sync::Arc};
use async_trait::async_trait;
use polywrap_core::env::{Envs,Env};
use serde_json::Value;

use crate::error::PluginError;

pub trait PluginWithEnv {
    fn set_env(&mut self, envs: Envs);
    fn get_env(&self, key: String) -> Option<&Env>;
}

#[async_trait]
pub trait PluginModule: Send + Sync {
    async fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &Value,
        invoker: Arc<dyn polywrap_core::invoke::Invoker>,
    ) -> Result<Value, PluginError>;
}
