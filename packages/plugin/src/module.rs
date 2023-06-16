use std::{fmt::Debug, sync::Arc};

use polywrap_core::{invoker::Invoker};

use crate::error::PluginError;

pub trait PluginModule: Send + Sync + Debug {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        env: Option<&[u8]>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, PluginError>;
}
