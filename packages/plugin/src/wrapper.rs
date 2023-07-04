use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use polywrap_core::{
    error::Error,
    invoker::Invoker,
    wrapper::{GetFileOptions, Wrapper},
};
use polywrap_msgpack_serde::to_vec;

use crate::module::PluginModule;

#[derive(Debug)]
pub struct PluginWrapper<T: PluginModule> {
    instance: Arc<Mutex<T>>,
}

impl<T: PluginModule> PluginWrapper<T> {
    pub fn new(instance: Arc<Mutex<T>>) -> Self {
        Self { instance }
    }
}

impl<T: PluginModule + 'static> Wrapper for PluginWrapper<T> {
    fn invoke(
        &self,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, Error> {
        let args = match args {
            Some(args) => args.to_vec(),
            None => to_vec(&{}).unwrap(),
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

impl<T: PluginModule> PartialEq for PluginWrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
