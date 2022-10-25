use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    invoke::{InvokeOptions, Invoker},
    loader::Loader,
    uri_resolution_context::UriResolutionContext,
    wrapper::Wrapper,
};

use crate::wrapper_loader::WrapperLoader;

#[derive(Clone)]
pub struct WrapperInvoker {
    loader: WrapperLoader,
}

impl WrapperInvoker {
    pub fn new(loader: WrapperLoader) -> Self {
        Self { loader }
    }
}

#[async_trait(?Send)]
impl Invoker for WrapperInvoker {
    async fn invoke_wrapper(
        &self,
        options: &InvokeOptions,
        wrapper: Box<dyn Wrapper>,
    ) -> Result<Vec<u8>, Error> {
        let result = wrapper.invoke(options, Arc::new(self.clone()));

        if result.is_err() {
            return Err(Error::InvokeError(format!(
                "Failed to invoke wrapper: {}",
                result.err().unwrap()
            )));
        };

        let result = result.unwrap();

        Ok(result)
    }

    async fn invoke(&self, options: &InvokeOptions) -> Result<Vec<u8>, Error> {
        let empty_res_context = UriResolutionContext::new();
        let resolution_context = match &options.resolution_context {
            None => &empty_res_context,
            Some(ctx) => ctx,
        };

        let uri = options.uri;

        let load_wrapper_result = self
            .loader
            .load_wrapper(uri, Some(resolution_context))
            .await;

        if load_wrapper_result.is_err() {
            return Err(Error::InvokeError(format!(
                "Failed to load wrapper: {}",
                load_wrapper_result.err().unwrap()
            )));
        };

        let wrapper = load_wrapper_result.unwrap();

        let invoke_opts = InvokeOptions {
            uri,
            args: options.args,
            method: options.method,
            resolution_context: options.resolution_context,
            env: None,
        };

        let invoke_result = self.invoke_wrapper(&invoke_opts, wrapper).await;

        if invoke_result.is_err() {
            return Err(Error::InvokeError(format!(
                "Failed to invoke wrapper: {}",
                invoke_result.err().unwrap()
            )));
        };

        Ok(invoke_result.unwrap())
    }
}
