use polywrap_client::core::{
    client::UriRedirect,
    package::WrapPackage,
    resolvers::{
        uri_resolution_context::UriPackageOrWrapper, uri_resolver::UriResolver,
        uri_resolver_like::UriResolverLike as InnerUriResolverLike,
    },
    wrapper::Wrapper,
};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use crate::loader::FFILoader;

pub trait FFIUriResolver: Send + Sync + Debug {
    fn ffi_try_resolve_uri(&self, uri: &str, loader: FFILoader) -> UriPackageOrWrapper;
}

impl UriResolver for Box<dyn FFIUriResolver> {
    fn try_resolve_uri(
        &self,
        uri: &polywrap_client::core::uri::Uri,
        loader: Arc<dyn polywrap_client::core::loader::Loader>,
        _: &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
        let loader = FFILoader::new(loader);
        Ok(self.ffi_try_resolve_uri(&uri.to_string(), loader))
    }
}

pub enum UriResolverLike {
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
        resolvers: Vec<UriResolverLike>,
    },
}

impl From<UriResolverLike> for InnerUriResolverLike {
    fn from(value: UriResolverLike) -> Self {
        match value {
            UriResolverLike::Resolver { resolver } => {
                InnerUriResolverLike::Resolver(Arc::from(resolver))
            }
            UriResolverLike::Redirect { from, to } => InnerUriResolverLike::Redirect(UriRedirect {
                from: from.try_into().unwrap(),
                to: to.try_into().unwrap(),
            }),
            UriResolverLike::Package { uri, package } => InnerUriResolverLike::Package(
                uri.try_into().unwrap(),
                Arc::new(Mutex::new(package)),
            ),
            UriResolverLike::Wrapper { uri, wrapper } => InnerUriResolverLike::Wrapper(
                uri.try_into().unwrap(),
                Arc::new(Mutex::new(wrapper)),
            ),
            UriResolverLike::ResolverLike { resolvers } => InnerUriResolverLike::ResolverLike(
                resolvers
                    .into_iter()
                    .map(|resolver_like| resolver_like.into())
                    .collect(),
            ),
        }
    }
}
