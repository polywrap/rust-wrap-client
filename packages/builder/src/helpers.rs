use std::sync::Arc;

use polywrap_resolvers::static_resolver::{StaticResolver, StaticResolverLike};

use crate::types::BuilderConfig;
use polywrap_core::{client::ClientConfig, resolution::uri_resolver::UriResolver};
use polywrap_resolver_extensions::extendable_uri_resolver::ExtendableUriResolver;
use polywrap_resolvers::resolution_result_cache_resolver::ResolutionResultCacheResolver;
use polywrap_resolvers::{
    recursive_resolver::RecursiveResolver,
    resolver_vec,
};

pub fn build_resolver(builder: BuilderConfig) -> ClientConfig {
    let mut static_resolvers: Vec<StaticResolverLike> = vec![];

    if let Some(wrappers) = &builder.wrappers {
        for (uri, w) in wrappers {
            static_resolvers.push(StaticResolverLike::Wrapper(uri.clone(), w.clone()));
        }
    }

    if let Some(packages) = &builder.packages {
        for (uri, p) in packages {
            static_resolvers.push(StaticResolverLike::Package(uri.clone(), p.clone()));
        }
    }

    if let Some(redirects) = &builder.redirects {
        for r in redirects {
            static_resolvers.push(StaticResolverLike::Redirect(r.clone()));
        }
    }

    ClientConfig {
        envs: builder.envs.clone(),
        interfaces: builder.interfaces.clone(),
        resolver: Arc::new(RecursiveResolver::from(
            Box::from(ResolutionResultCacheResolver::from(resolver_vec![
                StaticResolver::from(static_resolvers),
                ExtendableUriResolver::new(None),
            ])) as Box<dyn UriResolver>,
        )),
    }
}
