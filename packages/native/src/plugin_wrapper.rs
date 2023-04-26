use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use polywrap_client::core::wrapper::Wrapper;
use polywrap_plugin::module::PluginModule;
use polywrap_plugin::wrapper::PluginWrapper;

use crate::invoker::FFIInvoker;

pub trait FFIPluginModule: Send + Sync + Debug {
    fn __wrap_invoke(
        &self,
        method_name: &str,
        params: &[u8],
        env: Option<String>,
        invoker: FFIInvoker,
    ) -> Vec<u8>;
}

#[derive(Debug)]
pub struct FFIPluginModuleWrapper(Box<dyn FFIPluginModule>);

impl PluginModule for FFIPluginModuleWrapper {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        env: Option<polywrap_client::core::env::Env>,
        invoker: Arc<dyn polywrap_client::core::invoke::Invoker>,
    ) -> Result<Vec<u8>, polywrap_plugin::error::PluginError> {
        let env = env.map(|env| env.to_string());

        Ok(self.0.__wrap_invoke(method_name, params, env, invoker.into()))
    }
}

pub struct FFIPluginWrapper {
    pub inner_plugin: Arc<Mutex<Box<dyn Wrapper>>>,
}

impl FFIPluginWrapper {
    pub fn new(plugin_module_wrapper: FFIPluginModuleWrapper) -> FFIPluginWrapper {
        let plugin_module = Arc::new(Mutex::new(
            Box::new(plugin_module_wrapper) as Box<dyn PluginModule>
        ));

        let plugin_wrapper = PluginWrapper::new(plugin_module);

        FFIPluginWrapper {
            inner_plugin: Arc::new(Mutex::new(Box::new(plugin_wrapper) as Box<dyn Wrapper>)),
        }
    }
}
