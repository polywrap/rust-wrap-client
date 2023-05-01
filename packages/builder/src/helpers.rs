use std::sync::Arc;

use polywrap_core::{
    client::ClientConfig,
    resolvers::{
        recursive_resolver::RecursiveResolver,
        static_resolver::{StaticResolver, StaticResolverLike},
        uri_resolver::UriResolver,
        uri_resolver_like::UriResolverLike,
    },
};
use polywrap_resolvers::extendable_uri_resolver::ExtendableUriResolver;
use serde_json::Value;

use crate::types::BuilderConfig;

pub fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (&mut Value::Object(ref mut a), Value::Object(b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

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
    let static_resolver = StaticResolver::from(static_resolvers);
    let extendable_resolver = ExtendableUriResolver::new(None);

    let resolvers = vec![
        UriResolverLike::Resolver(Arc::new(static_resolver)),
        UriResolverLike::Resolver(Arc::new(extendable_resolver)),
    ];

    ClientConfig {
        envs: builder.envs.clone(),
        interfaces: builder.interfaces.clone(),
        resolver: Arc::new(RecursiveResolver::from(resolvers)) as Arc<dyn UriResolver>,
    }
}
