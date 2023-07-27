use core::fmt;
use polywrap_core::{
    error::Error,
    invoker::Invoker,
    macros::uri,
    resolution::{
        uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
        uri_resolver::UriResolver,
    },
    uri::Uri,
};
use polywrap_resolvers::uri_resolver_aggregator_base::UriResolverAggregatorBase;
use std::sync::{Arc, Mutex};

use crate::uri_resolver_wrapper::UriResolverWrapper;

pub struct ExtendableUriResolver {
    name: Option<String>,
}

impl ExtendableUriResolver {
    pub fn new(name: Option<String>) -> Self {
        ExtendableUriResolver { name }
    }
}

impl UriResolverAggregatorBase for ExtendableUriResolver {
    fn get_resolver_name(&self) -> Option<String> {
        self.name.clone()
    }

    fn get_uri_resolvers(
        &self,
        _: &Uri,
        invoker: &dyn Invoker,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<Vec<Arc<dyn UriResolver>>, Error> {
        let implementations =
            invoker.get_implementations(&uri!("wrapscan.io/polywrap/uri-resolver@1.0"))?;

        let resolvers = implementations
            .into_iter()
            .filter_map(|implementation| {
                if !resolution_context
                    .lock()
                    .unwrap()
                    .is_resolving(&implementation)
                {
                    let wrapper = Arc::new(UriResolverWrapper::new(implementation));
                    return Some(wrapper as Arc<dyn UriResolver>);
                }

                None
            })
            .collect::<Vec<Arc<dyn UriResolver>>>();

        Ok(resolvers)
    }

    fn get_step_description(&self, _: &Uri, _: &Result<UriPackageOrWrapper, Error>) -> String {
        if let Some(name) = self.get_resolver_name() {
            name
        } else {
            "ExtendableUriResolver".to_string()
        }
    }
}

impl UriResolver for ExtendableUriResolver {
    fn try_resolve_uri(
        &self,
        uri: &Uri,
        invoker: Arc<dyn Invoker>,
        resolution_context: Arc<Mutex<UriResolutionContext>>,
    ) -> Result<UriPackageOrWrapper, Error> {
        let resolvers =
            self.get_uri_resolvers(uri, invoker.as_ref(), resolution_context.clone())?;

        if resolvers.is_empty() {
            let uri = UriPackageOrWrapper::Uri(uri.clone());
            return Ok(uri);
        }

        self.try_resolve_uri_with_resolvers(uri, invoker, resolvers, resolution_context)
    }
}

impl fmt::Debug for ExtendableUriResolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExtendableUriResolver",)
    }
}
