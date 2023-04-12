use std::{sync::{Arc, Mutex}, fmt::{Debug}};

use polywrap_core::{uri::Uri, invoke::Invoker, wrapper::{Wrapper, GetFileOptions}, resolvers::uri_resolution_context::UriResolutionContext, env::Env};
use polywrap_msgpack::extensions::generic_map::convert_msgpack_to_json;


use crate::module::{PluginModule};

type PluginModuleInstance = Arc<Mutex<Box<dyn (PluginModule)>>>;

#[derive(Debug)]
pub struct PluginWrapper {
    instance: PluginModuleInstance,
}

impl PluginWrapper {
    pub fn new(
        instance: PluginModuleInstance,
    ) -> Self {
        Self { instance }
    }
}

impl Wrapper for PluginWrapper {
    fn invoke(
        &mut self,
        invoker: Arc<dyn Invoker>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        _: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        if let Some(e) = env {
            self.instance.lock().unwrap().set_env(e);
        };

        let args = match args {
            Some(args) => args.to_vec(),
            None => vec![],
        };

        let result = self
            .instance
            .lock().unwrap()
            ._wrap_invoke(method, &args, invoker);

        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(crate::error::PluginError::InvocationError {
                uri: uri.to_string(),
                method: method.to_string(),
                // Decode the args from msgpack to JSON for better error logging
                args: convert_msgpack_to_json(polywrap_msgpack::decode::<polywrap_msgpack::Value>(&args).unwrap()).to_string(),
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
