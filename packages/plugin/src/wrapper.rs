use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
    invoke::{InvokeOptions, Invoker},
    wrapper::{GetFileOptions, Wrapper},
};

use crate::module::PluginModule;

pub struct PluginWrapper {
    instance: Arc<dyn PluginModule>,
}

impl PluginWrapper {
    pub fn new(instance: Arc<dyn (PluginModule)>) -> Self {
        Self { instance }
    }
}

#[async_trait]
impl Wrapper for PluginWrapper {
    async fn invoke(
        &self,
        options: &InvokeOptions,
        invoker: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        let args = match options.args {
            Some(args) => match args {
                polywrap_core::invoke::InvokeArgs::Msgpack(value) => {
                    polywrap_msgpack::encode(value)
                        .map_err(|e| polywrap_core::error::Error::MsgpackError(e.to_string()))?
                }
                polywrap_core::invoke::InvokeArgs::UIntArray(arr) => arr.clone(),
            },
            None => vec![],
        };

        let json_args: serde_json::Value = polywrap_msgpack::decode(args.as_slice())
            .map_err(|e| polywrap_core::error::Error::MsgpackError(e.to_string()))?;

        let result = self
            .instance
            .clone()
            ._wrap_invoke(options.method, &json_args, invoker);

        match result {
            Ok(result) => Ok(rmp_serde::encode::to_vec(&result)
                .map_err(|e| polywrap_core::error::Error::MsgpackError(e.to_string()))?),
            Err(e) => Err(polywrap_core::error::Error::PluginError {
                uri: options.uri.to_string(),
                method: options.method.to_string(),
                args: json_args.to_string(),
                exception: e.to_string(),
            }),
        }
    }
    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, polywrap_core::error::Error> {
        unimplemented!("client.get_file(...) is not implemented for Plugins.")
    }
}
