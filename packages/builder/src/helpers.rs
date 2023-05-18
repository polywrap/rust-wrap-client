use std::sync::Arc;

use polywrap_core::{
    client::ClientConfig,
    resolution::{
        uri_resolver::UriResolver,
    },
};
use polywrap_resolvers::{
    extendable_uri_resolver::ExtendableUriResolver, 
    static_resolver::{StaticResolverLike, StaticResolver}, recursive_resolver::RecursiveResolver, resolver_vec
};
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

    ClientConfig {
        envs: builder.envs.clone(),
        interfaces: builder.interfaces.clone(),
        resolver: Arc::new(RecursiveResolver::from(resolver_vec![
            StaticResolver::from(static_resolvers),
            ExtendableUriResolver::new(None),
        ])),
    }
}
