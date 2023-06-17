use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use polywrap_core::{
    error::Error,
    invoker::Invoker,
    wrapper::{GetFileOptions, Wrapper},
};
use polywrap_msgpack::msgpack;

use crate::module::PluginModule;

type PluginModuleInstance = Arc<Mutex<Box<dyn (PluginModule)>>>;

#[derive(Debug)]
pub struct PluginWrapper {
    instance: PluginModuleInstance,
}

impl PluginWrapper {
    pub fn new(instance: PluginModuleInstance) -> Self {
        Self { instance }
    }
}

impl Wrapper for PluginWrapper {
    fn invoke(
        &self,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        invoker: Arc<dyn Invoker>,
        _: Option<Box<dyn Fn(String) + Send + Sync>>,
    ) -> Result<Vec<u8>, Error> {
        let args = match args {
            Some(args) => args.to_vec(),
            None => msgpack!({}),
        };

        let result = self
            .instance
            .lock()
            .unwrap()
            ._wrap_invoke(method, &args, env, invoker);

        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(crate::error::PluginError::InvocationError {
                exception: e.to_string(),
            }
            .into()),
        }
    }
    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, polywrap_core::error::Error> {
        unimplemented!("client.get_file(...) is not implemented for Plugins.")
    }
}

impl PartialEq for PluginWrapper {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
