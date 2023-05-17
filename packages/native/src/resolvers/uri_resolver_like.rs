use polywrap_client::core::{
    client::UriRedirect,
    resolvers::{uri_resolver::UriResolver, uri_resolver_like::UriResolverLike},
    uri::Uri,
};
use std::sync::Arc;

use crate::{package::FFIWrapPackage, wrapper::FFIWrapper, uri::FFIUri};

use super::ffi_resolver::{FFIUriResolver, FFIUriResolverWrapper};

pub struct FFIUriResolverLikeRedirectVariant {
    from: Arc<FFIUri>,
    to: Arc<FFIUri>,
}

impl FFIUriResolverLikeRedirectVariant {
    pub fn new(from: Arc<FFIUri>, to: Arc<FFIUri>) -> FFIUriResolverLikeRedirectVariant {
        FFIUriResolverLikeRedirectVariant { from, to }
    }
}

pub struct FFIUriResolverLikePackageVariant {
    uri: Arc<FFIUri>,
    package: Arc<FFIWrapPackage>,
}

impl FFIUriResolverLikePackageVariant {
    pub fn new(uri: Arc<FFIUri>, package: Arc<FFIWrapPackage>) -> FFIUriResolverLikePackageVariant {
        FFIUriResolverLikePackageVariant { uri, package }
    }
}

pub struct FFIUriResolverLikeWrapperVariant {
    uri: Arc<FFIUri>,
    wrapper: Arc<FFIWrapper>,
}

impl FFIUriResolverLikeWrapperVariant {
    pub fn new(uri: Arc<FFIUri>, wrapper: Arc<FFIWrapper>) -> FFIUriResolverLikeWrapperVariant {
        FFIUriResolverLikeWrapperVariant { uri, wrapper }
    }
}

pub struct FFIUriResolverLikeResolverVariant {
    resolver: Arc<dyn FFIUriResolver>,
}

impl FFIUriResolverLikeResolverVariant {
    pub fn new(resolver: Box<dyn FFIUriResolver>) -> FFIUriResolverLikeResolverVariant {
        FFIUriResolverLikeResolverVariant {
            resolver: Arc::from(resolver),
        }
    }
}

pub struct FFIUriResolverLikeResolverLikeVariant {
    resolver_like: Vec<Arc<FFIUriResolverLike>>,
}

impl FFIUriResolverLikeResolverLikeVariant {
    pub fn new(resolver_like: Vec<Arc<FFIUriResolverLike>>) -> FFIUriResolverLikeResolverLikeVariant {
        FFIUriResolverLikeResolverLikeVariant { resolver_like }
    }
}

#[derive(Clone)]
pub struct FFIUriResolverLike {
    kind: FFIUriResolverLikeKind,
    resolver: Option<Arc<FFIUriResolverLikeResolverVariant>>,
    redirect: Option<Arc<FFIUriResolverLikeRedirectVariant>>,
    wrapper: Option<Arc<FFIUriResolverLikeWrapperVariant>>,
    package: Option<Arc<FFIUriResolverLikePackageVariant>>,
    resolver_like: Option<Arc<FFIUriResolverLikeResolverLikeVariant>>,
}

impl FFIUriResolverLike {
    pub fn get_kind(&self) -> FFIUriResolverLikeKind {
        self.kind.clone()
    }

    pub fn new_resolver(resolver: Arc<FFIUriResolverLikeResolverVariant>) -> FFIUriResolverLike {
        FFIUriResolverLike {
            kind: FFIUriResolverLikeKind::_Resolver,
            resolver: Some(resolver),
            redirect: None,
            wrapper: None,
            package: None,
            resolver_like: None,
        }
    }
    pub fn new_redirect(redirect: Arc<FFIUriResolverLikeRedirectVariant>) -> FFIUriResolverLike {
        FFIUriResolverLike {
            kind: FFIUriResolverLikeKind::_Redirect,
            resolver: None,
            redirect: Some(redirect),
            wrapper: None,
            package: None,
            resolver_like: None,
        }
    }
    pub fn new_wrapper(wrapper: Arc<FFIUriResolverLikeWrapperVariant>) -> FFIUriResolverLike {
        FFIUriResolverLike {
            kind: FFIUriResolverLikeKind::_Wrapper,
            resolver: None,
            redirect: None,
            wrapper: Some(wrapper),
            package: None,
            resolver_like: None,
        }
    }
    pub fn new_package(package: Arc<FFIUriResolverLikePackageVariant>) -> FFIUriResolverLike {
        FFIUriResolverLike {
            kind: FFIUriResolverLikeKind::_Package,
            resolver: None,
            redirect: None,
            wrapper: None,
            package: Some(package),
            resolver_like: None,
        }
    }
    pub fn new_resolver_like(resolver_like: Arc<FFIUriResolverLikeResolverLikeVariant>) -> FFIUriResolverLike {
        FFIUriResolverLike {
            kind: FFIUriResolverLikeKind::_ResolverLike,
            resolver: None,
            redirect: None,
            wrapper: None,
            package: None,
            resolver_like: Some(resolver_like),
        }
    }

    pub fn get_resolver(&self) -> Option<Arc<FFIUriResolverLikeResolverVariant>> {
        self.resolver.clone()
    }
    pub fn get_redirect(&self) -> Option<Arc<FFIUriResolverLikeRedirectVariant>> {
        self.redirect.clone()
    }
    pub fn get_wrapper(&self) -> Option<Arc<FFIUriResolverLikeWrapperVariant>> {
        self.wrapper.clone()
    }
    pub fn get_package(&self) -> Option<Arc<FFIUriResolverLikePackageVariant>> {
        self.package.clone()
    }
    pub fn get_resolver_like(&self) -> Option<Arc<FFIUriResolverLikeResolverLikeVariant>> {
        self.resolver_like.clone()
    }
}

#[derive(Clone)]
pub enum FFIUriResolverLikeKind {
    _Resolver,
    _Redirect,
    _Package,
    _Wrapper,
    _ResolverLike,
}

impl From<FFIUriResolverLike> for UriResolverLike {
    fn from(value: FFIUriResolverLike) -> Self {
        match value.get_kind() {
            FFIUriResolverLikeKind::_Resolver => {
                let inner_resolver = value.get_resolver().unwrap().resolver.clone();
                let resolver =
                    Arc::new(FFIUriResolverWrapper::new(inner_resolver)) as Arc<dyn UriResolver>;
                UriResolverLike::Resolver(resolver)
            }
            FFIUriResolverLikeKind::_Redirect => {
                let redirect = value.get_redirect().unwrap();
                UriResolverLike::Redirect(UriRedirect {
                    from: redirect.from.0.clone(),
                    to: redirect.to.0.clone(),
                })
            }
            FFIUriResolverLikeKind::_Package => {
                let package = value.get_package().unwrap();
                UriResolverLike::Package(package.uri.0.clone(), package.package.0.clone())
            }
            FFIUriResolverLikeKind::_Wrapper => {
                let wrapper = value.get_wrapper().unwrap();
                UriResolverLike::Wrapper(wrapper.uri.0.clone(), wrapper.wrapper.0.clone())
            }
            FFIUriResolverLikeKind::_ResolverLike => UriResolverLike::ResolverLike(
                value
                    .get_resolver_like()
                    .unwrap()
                    .resolver_like
                    .clone()
                    .into_iter()
                    .map(|resolver_like| resolver_like.as_ref().clone().into())
                    .collect(),
            ),
        }
    }
}
