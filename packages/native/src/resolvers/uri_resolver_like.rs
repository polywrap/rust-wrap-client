use std::sync::{Arc, Mutex};

use polywrap_client::core::{
    client::UriRedirect,
    package::WrapPackage,
    resolvers::{
        uri_resolver::UriResolver,
        uri_resolver_like::UriResolverLike as InnerUriResolverLike,
    },
    wrapper::Wrapper,
};

pub enum UriResolverLike {
    Resolver(Box<dyn UriResolver>),
    Redirect(UriRedirect),
    Package(String, Box<dyn WrapPackage>),
    Wrapper(String, Box<dyn Wrapper>),
    ResolverLike(Vec<UriResolverLike>),
}

impl From<UriResolverLike> for InnerUriResolverLike {
    fn from(value: UriResolverLike) -> Self {
        match value {
            UriResolverLike::Resolver(resolver) => {
                InnerUriResolverLike::Resolver(Arc::from(resolver))
            }
            UriResolverLike::Redirect(redirect) => {
                InnerUriResolverLike::Redirect(redirect)
            }
            UriResolverLike::Package(uri, package) => InnerUriResolverLike::Package(
                uri.try_into().unwrap(),
                Arc::new(Mutex::new(package)),
            ),
            UriResolverLike::Wrapper(uri, wrapper) => InnerUriResolverLike::Wrapper(
                uri.try_into().unwrap(),
                Arc::new(Mutex::new(wrapper)),
            ),
            UriResolverLike::ResolverLike(resolver_likes) => {
                InnerUriResolverLike::ResolverLike(
                    resolver_likes
                        .into_iter()
                        .map(|resolver_like| resolver_like.into())
                        .collect(),
                )
            }
        }
    }
}

