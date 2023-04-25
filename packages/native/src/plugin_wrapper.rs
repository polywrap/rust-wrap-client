use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use polywrap_client::core::wrapper::Wrapper;
use polywrap_plugin::module::PluginModule;
use polywrap_plugin::wrapper::PluginWrapper;

use crate::client::FFIPolywrapClient;

pub trait FFIPluginModule: Send + Sync + Debug {
    fn invoke(
        &self,
        method_name: &str,
        params: &[u8],
        env: Option<String>,
        invoker: FFIPolywrapClient,
    ) -> Vec<u8>;
}

impl PluginModule for Box<dyn FFIPluginModule> {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        env: Option<polywrap_client::core::env::Env>,
        invoker: Arc<dyn polywrap_client::core::invoke::Invoker>,
    ) -> Result<Vec<u8>, polywrap_plugin::error::PluginError> {
        let env = env.map(|env| env.to_string());

        Ok(self.invoke(method_name, params, env, invoker.into()))
    }
}

pub struct FFIPluginWrapper {
    pub inner_plugin: Arc<Mutex<Box<dyn Wrapper>>>,
}

impl FFIPluginWrapper {
    pub fn new(plugin_module: Box<dyn FFIPluginModule>) -> FFIPluginWrapper {
        let plugin_wrapper = PluginWrapper::new(
          Arc::new(
            Mutex::new(
              // TODO: additional level of indirection necessary?
              Box::new(plugin_module) as Box<dyn PluginModule>
            )
          )
        );

        FFIPluginWrapper {
            inner_plugin: Arc::new(Mutex::new(Box::new(plugin_wrapper) as Box<dyn Wrapper>)),
        }
    }
}
