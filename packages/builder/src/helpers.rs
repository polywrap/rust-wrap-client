use std::sync::Arc;

use polywrap_core::{
    client::ClientConfig,
    resolution::{
        uri_resolver::UriResolver,
    },
};
use polywrap_resolver_extensions::extendable_uri_resolver::ExtendableUriResolver;
use polywrap_resolvers::{
    static_resolver::{StaticResolverLike, StaticResolver}, recursive_resolver::RecursiveResolver, resolver_vec
};
use polywrap_resolvers::resolution_result_cache_resolver::ResolutionResultCacheResolver;
use crate::types::BuilderConfig;

pub fn build_resolver(builder: BuilderConfig) -> ClientConfig {
    let mut static_resolvers: Vec<StaticResolverLike> = vec![];

    if let Some(wrappers) = builder.wrappers {
        for (uri, w) in wrappers {
            static_resolvers.push(StaticResolverLike::Wrapper(uri, w));
        }
    }

    if let Some(packages) = builder.packages {
        for (uri, p) in packages {
            static_resolvers.push(StaticResolverLike::Package(uri, p));
        }
    }

    if let Some(redirects) = builder.redirects {
        for r in redirects {
            static_resolvers.push(StaticResolverLike::Redirect(r));
        }
    }

    ClientConfig {
        envs: builder.envs.clone(),
        interfaces: builder.interfaces.clone(),
        resolver: Arc::new(RecursiveResolver::from(
            Box::from(ResolutionResultCacheResolver::from(resolver_vec![
                StaticResolver::from(static_resolvers),
                ExtendableUriResolver::new(None),
            ])) as Box<dyn UriResolver>
        )),
    }
}
