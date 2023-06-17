use polywrap_resolvers::{
    static_resolver::{StaticResolverLike, StaticResolver} 
};

use crate::PolywrapClientConfig;
pub fn build_static_resolver(builder: &PolywrapClientConfig) -> Option<StaticResolver> {
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

    if static_resolvers.len() > 0 {
        Some(StaticResolver::from(static_resolvers))
    } else {
        None
    }
}
