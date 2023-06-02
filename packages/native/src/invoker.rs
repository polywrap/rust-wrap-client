use polywrap_client::core::{
    invoker::Invoker, resolution::uri_resolution_context::UriResolutionContext, uri::Uri,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{resolvers::resolution_context::FFIUriResolutionContext, uri::FFIUri};

pub trait FFIInvoker: Send + Sync {
    fn invoke_raw(
        &self,
        uri: Arc<FFIUri>,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<Arc<FFIUriResolutionContext>>,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error>;

    fn get_implementations(
        &self,
        uri: Arc<FFIUri>,
    ) -> Result<Vec<Arc<FFIUri>>, polywrap_client::core::error::Error>;

    fn get_interfaces(&self) -> Option<HashMap<String, Vec<Arc<FFIUri>>>>;

    fn get_env_by_uri(&self, uri: Arc<FFIUri>) -> Option<Vec<u8>>;
}

pub struct InvokerWrapping(pub Arc<dyn Invoker>);

impl FFIInvoker for InvokerWrapping {
    fn invoke_raw(
        &self,
        uri: Arc<FFIUri>,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<Arc<FFIUriResolutionContext>>,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
        self.0.invoke_raw(&uri.0, method, args, env, resolution_context.map(|ctx| ctx.0))
    }

    fn get_implementations(
        &self,
        uri: Arc<FFIUri>,
    ) -> Result<Vec<Arc<FFIUri>>, polywrap_client::core::error::Error> {
        let uris = self.0.get_implementations(&uri.0)?;
        let uris: Vec<Arc<FFIUri>> = uris.into_iter().map(|uri| Arc::new(FFIUri(uri.clone()))).collect();

        Ok(uris)
    }

    fn get_interfaces(&self) -> Option<HashMap<String, Vec<Arc<FFIUri>>>> {
        let interfaces = self.0.get_interfaces();
        let interfaces: Option<HashMap<String, Vec<Arc<FFIUri>>>> = interfaces.map(|interfaces| {
          interfaces.into_iter().map(|(key, value)| (
            key,
            value.into_iter().map(|uri| Arc::new(FFIUri(uri.clone()))).collect()
          )).collect()
        });

        interfaces
    }

    fn get_env_by_uri(&self, uri: Arc<FFIUri>) -> Option<Vec<u8>> {
        self.0.get_env_by_uri(&uri.0).map(|env| env.to_vec())
    }
}

pub struct FFIInvokerWrapping(pub Box<dyn FFIInvoker>);

impl Invoker for FFIInvokerWrapping {
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, polywrap_client::core::error::Error> {
        self.0.invoke_raw(
            Arc::new(FFIUri(uri.clone())),
            method,
            args,
            env,
            resolution_context.map(|ctx| Arc::new(FFIUriResolutionContext(ctx))),
        )
    }

    fn get_implementations(
        &self,
        uri: &Uri,
    ) -> Result<Vec<Uri>, polywrap_client::core::error::Error> {
        let uris = self.0.get_implementations(Arc::new(FFIUri(uri.clone())))?;
        let uris: Vec<Uri> = uris.into_iter().map(|uri| uri.0).collect();

        Ok(uris)
    }

    fn get_interfaces(
        &self,
    ) -> Option<polywrap_client::core::interface_implementation::InterfaceImplementations> {
        let interfaces = self.0.get_interfaces();
        let interfaces: Option<HashMap<String, Vec<Uri>>> = interfaces.map(|interfaces| {
          interfaces.into_iter().map(|(key, value)| (
            key,
            value.into_iter().map(|uri| uri.0).collect()
          )).collect()
        });

        interfaces
    }

    fn get_env_by_uri(&self, uri: &Uri) -> Option<&[u8]> {
        let env = self.0.get_env_by_uri(Arc::new(FFIUri(uri.clone())));
        env.as_deref()
    }
}
