use std::sync::{Arc, Mutex};

use polywrap_client::core::wrapper::{GetFileOptions, Wrapper};
use polywrap_plugin::{
    module::PluginModule, wrapper::PluginWrapper,
};
use std::fmt::Debug;

use crate::client::FFIPolywrapClient;

pub trait FFIWrapper {
    fn get_inner_wrapper(&self) -> Arc<Mutex<Box<dyn Wrapper>>>;

    fn invoke(
        &self,
        invoker: FFIPolywrapClient,
        uri: &str,
        method: &str,
        args: Option<Vec<u8>>,
        env: Option<String>,
    ) -> Vec<u8> {
        let args = if let Some(args) = &args {
            Some(args.as_slice())
        } else {
            None
        };

        let env = if let Some(env) = &env {
            Some(serde_json::from_str(env).unwrap())
        } else {
            None
        };

        self.get_inner_wrapper()
            .lock()
            .unwrap()
            .invoke(
                Arc::new(invoker),
                &uri.to_string().try_into().unwrap(),
                method,
                args,
                env,
                None,
            )
            .unwrap()
    }

    fn get_file(&self, options: &GetFileOptions) -> Vec<u8> {
        self.get_inner_wrapper()
            .lock()
            .unwrap()
            .get_file(options)
            .unwrap()
    }
}

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
        let env = if let Some(env) = env {
            Some(env.to_string())
        } else {
            None
        };

        Ok(self.invoke(method_name, params, env, invoker.into()))
    }
}

pub struct FFIPluginWrapper {
    pub inner_plugin: Arc<Mutex<Box<dyn Wrapper>>>,
}

impl FFIPluginWrapper {
    pub fn new(plugin_module: Box<dyn FFIPluginModule>) -> FFIPluginWrapper {
      let inner_module = Box::new(plugin_module) as Box<dyn PluginModule>;
      let inner_module_instance = Arc::new(Mutex::new(inner_module));

      FFIPluginWrapper {
        inner_plugin: Arc::new(
          Mutex::new(
            Box::new(
              PluginWrapper::new(inner_module_instance)
            )
          )
        )
      }
    }
}

impl FFIWrapper for FFIPluginWrapper {
    fn get_inner_wrapper(&self) -> Arc<Mutex<Box<dyn Wrapper>>> {
        self.inner_plugin.clone()
    }
}

pub struct FFIWasmWrapper {
  pub inner_plugin: Arc<Mutex<Box<dyn Wrapper>>>,
}