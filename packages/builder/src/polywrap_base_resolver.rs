use std::sync::Arc;

use polywrap_core::resolution::uri_resolver::UriResolver;
use polywrap_resolver_extensions::extendable_uri_resolver::ExtendableUriResolver;
use polywrap_resolvers::{
    recursive_resolver::RecursiveResolver,
    resolution_result_cache_resolver::{
        ResolutionResultCacheResolver, ResolutionResultCacheResolverOptions,
    },
    static_resolver::StaticResolver,
    uri_resolver_aggregator::UriResolverAggregator,
};

/// Constructs a URI Resolver based on a set of default rules used by the `Client`.
pub struct PolywrapBaseResolver {}

/// Options for the construction of URI Resolver based on a set of default rules used by the `Client`.
#[derive(Default)]
pub struct PolywrapBaseResolverOptions {
    pub static_resolver: Option<StaticResolver>,
    pub dynamic_resolvers: Option<Vec<Arc<dyn UriResolver>>>,
    pub cache_resolver_options: Option<ResolutionResultCacheResolverOptions>,
}

impl PolywrapBaseResolver {
    pub fn new(options: PolywrapBaseResolverOptions) -> Arc<dyn UriResolver> {
        let mut resolvers: Vec<Arc<dyn UriResolver>> = vec![];

        if let Some(static_resolver) = options.static_resolver {
            resolvers.push(Arc::new(static_resolver));
        }

        if let Some(dynamic_resolvers) = options.dynamic_resolvers {
            if dynamic_resolvers.len() > 0 {
                for dynamic_resolver in dynamic_resolvers {
                    resolvers.push(dynamic_resolver);
                }
            }
        }

        resolvers.push(Arc::new(ExtendableUriResolver::new(None)));

        Arc::new(RecursiveResolver::from(
            Box::from(ResolutionResultCacheResolver::new(
                Arc::new(UriResolverAggregator::from(resolvers)),
                options
                    .cache_resolver_options
                    .unwrap_or(ResolutionResultCacheResolverOptions::default()),
            )) as Box<dyn UriResolver>,
        ))
    }
}
