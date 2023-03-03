use std::sync::{Arc, Mutex};

use polywrap_core::{
    error::Error,
    invoke::{Invoker},
    loader::Loader,
    resolvers::uri_resolution_context::UriResolutionContext,
    wrapper::Wrapper, uri::Uri, env::{Env}, 
    interface_implementation::InterfaceImplementations
};

use crate::wrapper_loader::WrapperLoader;

#[derive(Clone)]
pub struct WrapperInvoker {
    pub loader: WrapperLoader,
}

impl WrapperInvoker {
    pub fn new(
        loader: WrapperLoader
    ) -> Self {
        Self { loader }
    }
} 

impl Invoker for WrapperInvoker {
    fn invoke_wrapper_raw(
        &self,
        wrapper: Arc<Mutex<dyn Wrapper>>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let result = wrapper
            .lock().unwrap().invoke(Arc::new(self.clone()), uri, method, args, env, resolution_context)
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        Ok(result)
    }

    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error> {
        let mut empty_res_context = UriResolutionContext::new();
        let mut resolution_context = match resolution_context {
            None => &mut empty_res_context,
            Some(ctx) => ctx,
        };

        let wrapper = self
            .loader
            .load_wrapper(uri, Some(&mut resolution_context))
            .map_err(|e| Error::LoadWrapperError(e.to_string()))?;

        let mut env = env;
        if env.is_none() {
            if let Some(e) = self.loader.get_env_by_uri(&uri.clone()) {
                let e = e.to_owned();
                env = Some(e);
            };
        }

        let invoke_result = self
            .invoke_wrapper_raw(wrapper, uri, method, args, env, Some(resolution_context))
            .map_err(|e| Error::InvokeError(e.to_string()))?;

        Ok(invoke_result)
    }

    fn get_implementations(&self, uri: Uri) -> Result<Vec<Uri>, Error> {
        polywrap_core::resolvers::helpers::get_implementations(
            uri,
            self.get_interfaces(),
            Box::new(self.loader.clone())
        )
    }

    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        if let Some(interfaces) = self.loader.interfaces.clone() {
            return Some(interfaces);
        }

        None
    }
}
