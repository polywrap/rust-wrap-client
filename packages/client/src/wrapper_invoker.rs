use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    invoke::{InvokeOptions, Invoker},
    loader::Loader,
    uri_resolution_context::UriResolutionContext,
    wrapper::Wrapper, uri::Uri, interface_implementation::InterfaceImplementations,
};

use crate::wrapper_loader::WrapperLoader;

#[derive(Clone)]
pub struct WrapperInvoker {
    loader: WrapperLoader,
    interfaces: Option<InterfaceImplementations>
}

impl WrapperInvoker {
    pub fn new(loader: WrapperLoader, interfaces: Option<InterfaceImplementations>) -> Self {
        Self { loader, interfaces }
    }
}

#[async_trait]
impl Invoker for WrapperInvoker {
    async fn invoke_wrapper(
        &self,
        options: &InvokeOptions,
        mut wrapper: Box<dyn Wrapper>,
    ) -> Result<Vec<u8>, Error> {
        let result = wrapper.as_mut()
            .invoke(options, Arc::new(self.clone()))
            .await
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        Ok(result)
    }

    async fn invoke(&self, options: &InvokeOptions) -> Result<Vec<u8>, Error> {
        let empty_res_context = UriResolutionContext::new();
        let resolution_context = match &options.resolution_context {
            None => &empty_res_context,
            Some(ctx) => ctx,
        };

        let uri = options.uri;

        let wrapper = self
            .loader
            .load_wrapper(uri, Some(resolution_context))
            .await
            .map_err(|e| Error::LoadWrapperError(e.to_string()))?;

        let invoke_opts = InvokeOptions {
            uri,
            args: options.args,
            method: options.method,
            resolution_context: options.resolution_context,
            env: options.env
        };

        let invoke_result = self
            .invoke_wrapper(&invoke_opts, wrapper)
            .await
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        Ok(invoke_result)
    }

    fn get_implementations(&self, uri: Uri) -> Result<Vec<Uri>, Error> {
        if let Some(interfaces) = &self.interfaces {
            let implementations_value = interfaces.get(&uri.uri);
            if let Some(implementations) = implementations_value {
                return Ok(implementations.clone());
            }
        }
        Ok(vec![])
    }
}
