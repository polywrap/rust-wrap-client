use std::{sync::Arc};
use async_trait::async_trait;
use polywrap_core::env::{Env};

use crate::error::PluginError;

pub trait PluginWithEnv {
    fn set_env(&mut self, env: Env);
    fn get_env(&self, key: String) -> Option<&Env>;
}

#[async_trait]
pub trait PluginModule: Send + Sync + PluginWithEnv {
    async fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        invoker: Arc<dyn polywrap_core::invoke::Invoker>,
    ) -> Result<Vec<u8>, PluginError>;
}
