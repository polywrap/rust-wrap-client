use polywrap_client::core::{
    client::UriRedirect,
    package::WrapPackage,
    resolvers::{
        uri_resolver::UriResolver,
        uri_resolver_like::UriResolverLike,
    },
    wrapper::Wrapper,
};
use std::sync::{Arc, Mutex};

pub enum FFIUriResolverLike {
    Resolver {
        resolver: Box<dyn UriResolver>,
    },
    Redirect {
        from: String,
        to: String,
    },
    Package {
        uri: String,
        package: Box<dyn WrapPackage>,
    },
    Wrapper {
        uri: String,
        wrapper: Box<dyn Wrapper>,
    },
    ResolverLike {
        resolvers: Vec<FFIUriResolverLike>,
    },
}

impl From<FFIUriResolverLike> for UriResolverLike {
    fn from(value: FFIUriResolverLike) -> Self {
        match value {
            FFIUriResolverLike::Resolver { resolver } => {
                UriResolverLike::Resolver(Arc::from(resolver))
            }
            FFIUriResolverLike::Redirect { from, to } => UriResolverLike::Redirect(UriRedirect {
                from: from.try_into().unwrap(),
                to: to.try_into().unwrap(),
            }),
            FFIUriResolverLike::Package { uri, package } => UriResolverLike::Package(
                uri.try_into().unwrap(),
                Arc::new(Mutex::new(package)),
            ),
            FFIUriResolverLike::Wrapper { uri, wrapper } => UriResolverLike::Wrapper(
                uri.try_into().unwrap(),
                Arc::new(Mutex::new(wrapper)),
            ),
            FFIUriResolverLike::ResolverLike { resolvers } => UriResolverLike::ResolverLike(
                resolvers
                    .into_iter()
                    .map(|resolver_like| resolver_like.into())
                    .collect(),
            ),
        }
    }
}
