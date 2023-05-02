use core::fmt;
use std::sync::Arc;
use polywrap_core::{
    resolvers::{
        uri_resolution_context::{
            UriResolutionContext,
            UriPackageOrWrapper
        },
        uri_resolver_aggregator_base::UriResolverAggregatorBase,
        uri_resolver::UriResolver
    },
    uri::Uri,
    error::Error, client::Client
};

use crate::uri_resolver_wrapper::UriResolverWrapper;

pub struct ExtendableUriResolver {
    name: Option<String>
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
        client: Arc<dyn Client>,
        resolution_context: &mut UriResolutionContext
    ) -> Result<Vec<Arc<dyn UriResolver>>, Error> {
        let implementations = client.get_implementations(
           Uri::try_from("wrap://ens/uri-resolver.core.polywrap.eth")?
        )?;

        let resolvers = implementations.into_iter().filter_map(|implementation| {
            if !resolution_context.is_resolving(&implementation) {
                let wrapper = Arc::new(UriResolverWrapper::new(implementation));
                return Some(wrapper as Arc<dyn UriResolver>);
            }

            None
        }).collect::<Vec<Arc<dyn UriResolver>>>();

        Ok(resolvers)
    }

    fn get_step_description(
        &self,
        _: &Uri,
        _: &Result<UriPackageOrWrapper, Error>,
    ) -> String {
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
        client: Arc<dyn Client>, 
        resolution_context: &mut UriResolutionContext
    ) -> Result<UriPackageOrWrapper, Error> {
        let resolvers = self.get_uri_resolvers(
            &uri.clone(),
            client.clone(),
            resolution_context
        )?;

        if resolvers.is_empty() {
            let uri = UriPackageOrWrapper::Uri(uri.clone());
            return Ok(uri);
        }

        self.try_resolve_uri_with_resolvers(
            &uri.clone(),
            client,
            resolvers,
            resolution_context
        )
    }
}

impl fmt::Debug for ExtendableUriResolver {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "ExtendableUriResolver", )
  }
}