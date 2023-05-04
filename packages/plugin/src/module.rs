use std::fmt::Debug;

use polywrap_core::env::Env;

use crate::error::PluginError;

pub trait PluginModule: Send + Sync + Debug {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        env: Option<&Env>,
        invoker: &dyn polywrap_core::invoke::Invoker,
    ) -> Result<Vec<u8>, PluginError>;
}
