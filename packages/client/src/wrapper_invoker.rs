use std::sync::Arc;

use async_trait::async_trait;
use polywrap_core::{
    error::Error,
    invoke::{Invoker, InvokeArgs},
    loader::Loader,
    uri_resolution_context::UriResolutionContext,
    wrapper::Wrapper, uri::Uri, env::{Env},
    interface_implementation::InterfaceImplementations
};
use tokio::sync::Mutex;

use crate::wrapper_loader::WrapperLoader;

#[derive(Clone)]
pub struct WrapperInvoker {
    pub loader: WrapperLoader,
    pub interfaces: Option<InterfaceImplementations>
}

impl WrapperInvoker {
    pub fn new(
        loader: WrapperLoader, 
        interfaces: Option<InterfaceImplementations>
    ) -> Self {
        Self { loader, interfaces }
    }
}

#[async_trait]
impl Invoker for WrapperInvoker {
    async fn invoke_wrapper(
        &self,
        wrapper: Arc<Mutex<dyn Wrapper>>,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let result = wrapper
            .lock()
            .await
            .invoke(Arc::new(self.clone()), uri, method, args, env, resolution_context)
            .await
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        Ok(result)
    }

    async fn invoke(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&InvokeArgs>,
        env: Option<Env>,
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

        let mut env = env;
        if env.is_none() {
            if let Some(e) = self.loader.get_env_by_uri(&uri.clone()) {
                let e = e.to_owned();
                env = Some(e);
            };
        }

        let invoke_result = self
            .invoke_wrapper(wrapper, uri, method, args, env, Some(resolution_context))
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
