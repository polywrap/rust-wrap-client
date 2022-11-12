use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    invoke::{Invoker, InvokeArgs},
    loader::Loader,
    uri_resolution_context::UriResolutionContext,
    wrapper::Wrapper, uri::Uri,
};
use tokio::sync::Mutex;

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

#[async_trait]
impl Invoker for WrapperInvoker {
    async fn invoke_wrapper(
        &self,
        wrapper: Arc<Mutex<Box<dyn Wrapper>>>,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let result = wrapper
            .lock()
            .await
            .invoke(Arc::new(self.clone()), uri, method, args, resolution_context)
            .await
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        Ok(result)
    }

    async fn invoke(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let mut empty_res_context = UriResolutionContext::new();
        let mut resolution_context = match resolution_context {
            None => &mut empty_res_context,
            Some(ctx) => ctx,
        };

        let uri = uri;

        let wrapper = self
            .loader
            .load_wrapper(uri, Some(&mut resolution_context))
            .await
            .map_err(|e| Error::LoadWrapperError(e.to_string()))?;

        let invoke_result = self
            .invoke_wrapper(Arc::new(Mutex::new(wrapper)), uri, method, args, Some(resolution_context))
            .await
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        Ok(invoke_result)
    }
}
