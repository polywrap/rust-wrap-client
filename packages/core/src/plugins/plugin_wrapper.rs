use std::sync::Arc;

use crate::{
    error::Error,
    invoke::{InvokeArgs, InvokeOptions, Invoker},
    plugins::PluginModule,
    wrapper::{GetFileOptions, Wrapper},
};
use async_trait::async_trait;

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
    ) -> Result<Vec<u8>, Error> {
        let args = match options.args {
            Some(args) => match args {
                InvokeArgs::Msgpack(value) => polywrap_msgpack::encode(value)
                    .map_err(|e| Error::MsgpackError(e.to_string()))?,
                InvokeArgs::UIntArray(arr) => arr.clone(),
            },
            None => vec![],
        };

        let json_args: serde_json::Value = polywrap_msgpack::decode(args.as_slice())
            .map_err(|e| Error::MsgpackError(e.to_string()))?;

        let result = self
            .instance
            .clone()
            ._wrap_invoke(options.method, &json_args, invoker);

        match result {
            Ok(result) => Ok(rmp_serde::encode::to_vec(&result)
                .map_err(|e| Error::MsgpackError(e.to_string()))?),
            Err(e) => Err(Error::PluginError {
                uri: options.uri.to_string(),
                method: options.method.to_string(),
                args: json_args.to_string(),
                exception: e.to_string(),
            }),
        }
    }
    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, Error> {
        unimplemented!("client.get_file(...) is not implemented for Plugins.")
    }
}
