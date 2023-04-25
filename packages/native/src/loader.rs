use polywrap_client::core::{loader::Loader, resolvers::uri_resolver::UriResolverHandler};
use std::sync::Arc;

pub struct FFILoader {
    inner_loader: Arc<dyn Loader>,
}

impl FFILoader {
    pub fn new(loader: Arc<dyn Loader>) -> FFILoader {
        FFILoader {
            inner_loader: loader,
        }
    }
}

impl Loader for FFILoader {
    fn load_wrapper(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        resolution_context: Option<
            &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
        >,
    ) -> Result<
        Arc<std::sync::Mutex<Box<dyn polywrap_client::core::wrapper::Wrapper>>>,
        polywrap_client::core::error::Error,
    > {
        self.inner_loader.load_wrapper(uri, resolution_context)
    }

    fn get_env_by_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
    ) -> Option<&polywrap_client::core::env::Env> {
        self.inner_loader.get_env_by_uri(uri)
    }

    fn get_invoker(
        &self,
    ) -> Result<Arc<dyn polywrap_client::core::invoke::Invoker>, polywrap_client::core::error::Error>
    {
        self.inner_loader.get_invoker()
    }
}

impl UriResolverHandler for FFILoader {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        resolution_context: Option<
            &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
        >,
    ) -> Result<
        polywrap_client::core::resolvers::uri_resolution_context::UriPackageOrWrapper,
        polywrap_client::core::error::Error,
    > {
        self.inner_loader.try_resolve_uri(uri, resolution_context)
    }
}

impl From<Box<dyn Loader>> for FFILoader {
    fn from(value: Box<dyn Loader>) -> Self {
        FFILoader::new(Arc::from(value))
    }
}
