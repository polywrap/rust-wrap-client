use polywrap_client::core::{
    client::UriRedirect,
    resolvers::{uri_resolver::UriResolver, uri_resolver_like::UriResolverLike},
    uri::Uri,
};
use std::sync::Arc;

use crate::{package::FFIWrapPackage, wrapper::FFIWrapper};

use super::ffi_resolver::{FFIUriResolver, FFIUriResolverWrapper};

pub struct RedirectVariant {
    from: Arc<Uri>,
    to: Arc<Uri>,
}

impl RedirectVariant {
    pub fn new(from: Arc<Uri>, to: Arc<Uri>) -> RedirectVariant {
        RedirectVariant { from, to }
    }
}

pub struct PackageVariant {
    uri: Arc<Uri>,
    package: Arc<FFIWrapPackage>,
}

impl PackageVariant {
    pub fn new(uri: Arc<Uri>, package: Arc<FFIWrapPackage>) -> PackageVariant {
        PackageVariant { uri, package }
    }
}

pub struct WrapperVariant {
    uri: Arc<Uri>,
    wrapper: Arc<FFIWrapper>,
}

impl WrapperVariant {
    pub fn new(uri: Arc<Uri>, wrapper: Arc<FFIWrapper>) -> WrapperVariant {
        WrapperVariant { uri, wrapper }
    }
}

pub struct ResolverVariant {
    resolver: Arc<dyn FFIUriResolver>,
}

impl ResolverVariant {
    pub fn new(resolver: Box<dyn FFIUriResolver>) -> ResolverVariant {
        ResolverVariant {
            resolver: Arc::from(resolver),
        }
    }
}

pub struct ResolverLikeVariant {
    resolver_like: Vec<FFIUriResolverLike>,
}

impl ResolverLikeVariant {
    pub fn new(resolver_like: Vec<FFIUriResolverLike>) -> ResolverLikeVariant {
        ResolverLikeVariant { resolver_like }
    }
}

#[derive(Clone)]
pub struct FFIUriResolverLike {
    kind: FFIUriResolverLikeKind,
    resolver: Option<Arc<ResolverVariant>>,
    redirect: Option<Arc<RedirectVariant>>,
    wrapper: Option<Arc<WrapperVariant>>,
    package: Option<Arc<PackageVariant>>,
    resolver_like: Option<Arc<ResolverLikeVariant>>,
}

impl FFIUriResolverLike {
    pub fn get_kind(&self) -> FFIUriResolverLikeKind {
        self.kind.clone()
    }

    pub fn new_resolver(resolver: Arc<ResolverVariant>) -> FFIUriResolverLike {
        FFIUriResolverLike {
            kind: FFIUriResolverLikeKind::Resolver,
            resolver: Some(resolver),
            redirect: None,
            wrapper: None,
            package: None,
            resolver_like: None,
        }
    }
    pub fn new_redirect(redirect: Arc<RedirectVariant>) -> FFIUriResolverLike {
        FFIUriResolverLike {
            kind: FFIUriResolverLikeKind::Redirect,
            resolver: None,
            redirect: Some(redirect),
            wrapper: None,
            package: None,
            resolver_like: None,
        }
    }
    pub fn new_wrapper(wrapper: Arc<WrapperVariant>) -> FFIUriResolverLike {
        FFIUriResolverLike {
            kind: FFIUriResolverLikeKind::Wrapper,
            resolver: None,
            redirect: None,
            wrapper: Some(wrapper),
            package: None,
            resolver_like: None,
        }
    }
    pub fn new_package(package: Arc<PackageVariant>) -> FFIUriResolverLike {
        FFIUriResolverLike {
            kind: FFIUriResolverLikeKind::Package,
            resolver: None,
            redirect: None,
            wrapper: None,
            package: Some(package),
            resolver_like: None,
        }
    }
    pub fn new_resolver_like(resolver_like: Arc<ResolverLikeVariant>) -> FFIUriResolverLike {
        FFIUriResolverLike {
            kind: FFIUriResolverLikeKind::ResolverLike,
            resolver: None,
            redirect: None,
            wrapper: None,
            package: None,
            resolver_like: Some(resolver_like),
        }
    }

    pub fn get_resolver(&self) -> Option<Arc<ResolverVariant>> {
        self.resolver.clone()
    }
    pub fn get_redirect(&self) -> Option<Arc<RedirectVariant>> {
        self.redirect.clone()
    }
    pub fn get_wrapper(&self) -> Option<Arc<WrapperVariant>> {
        self.wrapper.clone()
    }
    pub fn get_package(&self) -> Option<Arc<PackageVariant>> {
        self.package.clone()
    }
    pub fn get_resolver_like(&self) -> Option<Arc<ResolverLikeVariant>> {
        self.resolver_like.clone()
    }
}

#[derive(Clone)]
pub enum FFIUriResolverLikeKind {
    Resolver,
    Redirect,
    Package,
    Wrapper,
    ResolverLike,
}

impl From<FFIUriResolverLike> for UriResolverLike {
    fn from(value: FFIUriResolverLike) -> Self {
        match value.get_kind() {
            FFIUriResolverLikeKind::Resolver => {
                let inner_resolver = value.get_resolver().unwrap().resolver.clone();
                let resolver =
                    Arc::new(FFIUriResolverWrapper::new(inner_resolver)) as Arc<dyn UriResolver>;
                UriResolverLike::Resolver(resolver)
            }
            FFIUriResolverLikeKind::Redirect => {
                let redirect = value.get_redirect().unwrap();
                UriResolverLike::Redirect(UriRedirect {
                    from: redirect.from.as_ref().clone(),
                    to: redirect.to.as_ref().clone(),
                })
            }
            FFIUriResolverLikeKind::Package => {
                let package = value.get_package().unwrap();
                UriResolverLike::Package(package.uri.as_ref().clone(), package.package.0.clone())
            }
            FFIUriResolverLikeKind::Wrapper => {
                let wrapper = value.get_wrapper().unwrap();
                UriResolverLike::Wrapper(wrapper.uri.as_ref().clone(), wrapper.wrapper.0.clone())
            }
            FFIUriResolverLikeKind::ResolverLike => UriResolverLike::ResolverLike(
                value
                    .get_resolver_like()
                    .unwrap()
                    .resolver_like
                    .clone()
                    .into_iter()
                    .map(|resolver_like| resolver_like.into())
                    .collect(),
            ),
        }
    }
}
