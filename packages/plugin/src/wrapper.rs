use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use polywrap_core::{
    env::Env,
    invoke::Invoker,
    resolvers::uri_resolution_context::UriResolutionContext,
    uri::Uri,
    wrapper::{GetFileOptions, Wrapper},
};

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
        invoker: Arc<dyn Invoker>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&Env>,
        _: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        let args = match args {
            Some(args) => args.to_vec(),
            None => vec![],
        };

        let result = self
            .instance
            .lock()
            .unwrap()
            ._wrap_invoke(method, &args, env, invoker);

        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(crate::error::PluginError::InvocationError {
                uri: uri.to_string(),
                method: method.to_string(),
                // TODO: Add helper to decode the args from msgpack to JSON for better error logging
                args: polywrap_msgpack::decode::<polywrap_msgpack::Value>(&args)
                    .unwrap()
                    .to_string(),
                exception: e.to_string(),
            }
            .into()),
        }
    }
    fn get_file(&self, _: &dyn Client, _: &GetFileOptions) -> Result<Vec<u8>, polywrap_core::error::Error> {
        unimplemented!("client.get_file(...) is not implemented for Plugins.")
    }
}

impl PartialEq for PluginWrapper {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
