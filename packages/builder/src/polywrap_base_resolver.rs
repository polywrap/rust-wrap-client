use std::sync::Arc;

use polywrap_core::resolution::uri_resolver::UriResolver;
use polywrap_resolver_extensions::extendable_uri_resolver::ExtendableUriResolver;
use polywrap_resolvers::{static_resolver::StaticResolver, resolution_result_cache_resolver::{ResolutionResultCacheResolverOptions, ResolutionResultCacheResolver}, resolver_vec, recursive_resolver::RecursiveResolver, uri_resolver_aggregator::UriResolverAggregator};

pub struct PolywrapBaseResolver {
}

#[derive(Default)]
pub struct PolywrapBaseResolverOptions {
    pub static_resolver: Option<StaticResolver>,
    pub dynamic_resolvers: Option<Vec<Arc<dyn UriResolver>>>,
    pub cache_resolver_options: Option<ResolutionResultCacheResolverOptions>,
}

impl PolywrapBaseResolver {
    pub fn default() -> Arc<dyn UriResolver> {
        Arc::new(RecursiveResolver::from(
            Box::from(ResolutionResultCacheResolver::from(resolver_vec![
                ExtendableUriResolver::new(None),
            ])) as Box<dyn UriResolver>
        ))
    }

    pub fn new(options: PolywrapBaseResolverOptions) -> Arc<dyn UriResolver> {
        let mut resolvers: Vec<Arc<dyn UriResolver>> = vec![];

        if let Some(static_resolver) = options.static_resolver {
            resolvers.push(
                Arc::new(static_resolver)
            );
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
                options.cache_resolver_options.unwrap_or(ResolutionResultCacheResolverOptions::default()),
            )) as Box<dyn UriResolver>
        ))
    }
}
