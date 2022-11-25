use std::sync::Arc;

use polywrap_core::{
    resolvers::{
        uri_resolver::UriResolver, 
        uri_resolution_context::UriResolutionContext, helpers::get_implementations
    }, 
    uri::Uri, 
    loader::Loader,
    error::Error
};

use crate::uri_resolver_wrapper::UriResolverWrapper;

pub struct ExtendableUriResolver {
    name: Option<String>
}

impl ExtendableUriResolver {
    pub fn new(name: Option<String>) -> Self {
        ExtendableUriResolver { name }
    }

    pub fn resolver_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    async fn get_uri_resolvers(
        uri: Uri,
        loader: &dyn Loader,
        resolution_context: &mut UriResolutionContext
    ) -> Result<Vec<Arc<dyn UriResolver>>, Error> {
        let invoker = loader.get_invoker()?;
        let interfaces = invoker.lock().await.get_interfaces();
        let implementations = get_implementations(
           Uri::try_from("wrap://ens/uri-resolver.core.polywrap.eth")?,
           interfaces,
           Some(loader),
           Some(resolution_context)
        ).await?;

        let resolvers = implementations.into_iter().filter_map(|implementation| {
            if resolution_context.is_resolving(&implementation) {
                return Some(UriResolverWrapper::new(implementation));
            }

            None
        }).collect::<Vec<UriResolverWrapper>>();

        Ok(resolvers)
    }
}