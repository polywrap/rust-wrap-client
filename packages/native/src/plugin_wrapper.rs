use std::fmt::Debug;
use std::sync::{Arc};

use polywrap_client::core::invoker::Invoker;
use polywrap_plugin::module::PluginModule;

use crate::invoker::FFIInvoker;

pub trait FFIPluginModule: Send + Sync + Debug {
    fn wrap_invoke(
        &self,
        method_name: String,
        params: Vec<u8>,
        env: Option<String>,
        invoker: Arc<FFIInvoker>,
    ) -> Vec<u8>;
}

impl PluginModule for Box<dyn FFIPluginModule> {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        env: Option<&polywrap_client::core::env::Env>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, polywrap_plugin::error::PluginError> {
        let env = env.map(|env| env.to_string());
        let invoker = FFIInvoker {
            inner_invoker: invoker,
        };
        Ok(self.wrap_invoke(
          method_name.to_string(),
          params.to_vec(),
          env,
          Arc::new(invoker)
        ))
    }
}
