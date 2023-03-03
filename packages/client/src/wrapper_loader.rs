use std::sync::Arc;

use polywrap_core::{
    error::Error,
    loader::Loader,
    uri::Uri,
    resolvers::uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
    resolvers::uri_resolver::{UriResolver, UriResolverHandler},
    wrapper::Wrapper, env::{Envs, Env}, invoke::Invoker, interface_implementation::InterfaceImplementations,
};
use futures::lock::Mutex;

use crate::wrapper_invoker::WrapperInvoker;

#[derive(Clone)]
pub struct WrapperLoader {
    pub resolver: Arc<dyn UriResolver>,
    pub envs: Option<Envs>,
    pub interfaces: Option<InterfaceImplementations>
}

impl WrapperLoader {
    pub fn new(
        resolver: Arc<dyn UriResolver>, 
        envs: Option<Envs>,
        interfaces: Option<InterfaceImplementations>,
    ) -> Self {
        Self { resolver, envs, interfaces }
    }
}

impl UriResolverHandler for WrapperLoader {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<UriPackageOrWrapper, Error> {
        let uri_resolver = self.resolver.clone();
        let mut uri_resolver_context = UriResolutionContext::new();

        let resolution_context = match resolution_context {
            Some(ctx) => ctx,
            None => &mut uri_resolver_context,
        };

         uri_resolver
            .try_resolve_uri(uri, self, resolution_context)
    }
}

impl Loader for WrapperLoader {
    fn load_wrapper(
        &self,
        uri: &Uri,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Arc<Mutex<dyn Wrapper>>, Error> {
        let mut empty_res_context = UriResolutionContext::new();
        let mut resolution_ctx = match resolution_context {
            Some(ctx) => ctx,
            None => &mut empty_res_context,
        };

        let uri_package_or_wrapper = self
            .try_resolve_uri(uri, Some(&mut resolution_ctx))
            .map_err(|e| Error::ResolutionError(e.to_string()))?;

        match uri_package_or_wrapper {
            UriPackageOrWrapper::Uri(uri) => Err(Error::InvokeError(format!(
                "Failed to resolve wrapper: {}",
                uri
            ))),
            UriPackageOrWrapper::Wrapper(_, wrapper) => Ok(wrapper),
            UriPackageOrWrapper::Package(_, package) => {
                let wrapper = package
                    .try_lock().unwrap()
                    .create_wrapper().map_err(|e| Error::WrapperCreateError(e.to_string()))?;
                Ok(wrapper)
            }
        }
    }

    fn get_env_by_uri(&self, uri: &Uri) -> Option<&Env> {
        if let Some(envs) = &self.envs {
            return envs.get(&uri.uri);
        }

        None
    }

    fn get_invoker(&self) -> Result<Arc<dyn Invoker>, Error> {
        Ok(Arc::new(WrapperInvoker { 
            loader: self.to_owned()
        }))
    }
}
