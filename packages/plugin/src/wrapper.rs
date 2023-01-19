use std::{sync::Arc};

use async_trait::async_trait;
use futures::lock::Mutex;
use polywrap_core::{uri::Uri, invoke::Invoker, wrapper::{Wrapper, GetFileOptions}, resolvers::uri_resolution_context::UriResolutionContext, env::Env};

use crate::module::{PluginModule};

type PluginModuleInstance = Arc<Mutex<Box<dyn (PluginModule)>>>;

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

#[async_trait]
impl Wrapper for PluginWrapper {
    async fn invoke(
        &mut self,
        invoker: Arc<dyn Invoker>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        _: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        if let Some(e) = env {
            self.instance.lock().await.set_env(e);
        };

        let args = match args {
            Some(args) => args.to_vec(),
            None => vec![],
        };

        let json_args: serde_json::Value = polywrap_msgpack::decode(args.as_slice())?;

        let result = self
            .instance
            .lock()
            .await
            ._wrap_invoke(method, &json_args, invoker)
            .await;

        match result {
            Ok(result) => Ok(polywrap_msgpack::serialize(&result)?),
            Err(e) => Err(crate::error::PluginError::InvocationError {
                uri: uri.to_string(),
                method: method.to_string(),
                args: json_args.to_string(),
                exception: e.to_string(),
            }
            .into()),
        }
    }
    async fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, polywrap_core::error::Error> {
        unimplemented!("client.get_file(...) is not implemented for Plugins.")
    }
}