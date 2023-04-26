use polywrap_client::core::{
    client::UriRedirect,
    resolvers::{
        uri_resolver::UriResolver,
        uri_resolver_like::UriResolverLike,
    }, uri::Uri,
};
use std::sync::{Arc};

use crate::{package::FFIWrapPackage, wrapper::FFIWrapper};

use super::ffi_resolver::{FFIUriResolver, FFIUriResolverWrapper};

pub enum FFIUriResolverLike {
    Resolver {
        resolver: Box<dyn FFIUriResolver>,
    },
    Redirect {
        from: Arc<Uri>,
        to: Arc<Uri>,
    },
    Package {
        uri: Arc<Uri>,
        package: Arc<FFIWrapPackage>,
    },
    Wrapper {
        uri: Arc<Uri>,
        wrapper: Arc<FFIWrapper>,
    },
    ResolverLike {
        resolvers: Vec<FFIUriResolverLike>,
    },
}

impl From<FFIUriResolverLike> for UriResolverLike {
    fn from(value: FFIUriResolverLike) -> Self {
        match value {
            FFIUriResolverLike::Resolver { resolver } => {
                let ffi_uri_resolver_wrapper = Arc::new(FFIUriResolverWrapper::new(resolver)) as Arc<dyn UriResolver>;
                UriResolverLike::Resolver(ffi_uri_resolver_wrapper)
            }
            FFIUriResolverLike::Redirect { from, to } => UriResolverLike::Redirect(UriRedirect {
                from: from.as_ref().clone(),
                to: to.as_ref().clone(),
            }),
            FFIUriResolverLike::Package { uri, package } => UriResolverLike::Package(
                uri.as_ref().clone(),
                package.0.clone(),
            ),
            FFIUriResolverLike::Wrapper { uri, wrapper } => UriResolverLike::Wrapper(
                uri.as_ref().clone(),
                wrapper.0.clone(),
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
