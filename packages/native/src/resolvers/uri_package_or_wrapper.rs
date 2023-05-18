use std::sync::Arc;

use polywrap_client::core::{resolvers::uri_resolution_context::UriPackageOrWrapper};

use crate::{package::FFIWrapPackage, wrapper::{FFIWrapper, ExtWrapper}, uri::FFIUri};

#[derive(Clone)]
pub enum FFIUriPackageOrWrapperKind {
    _Uri,
    _Wrapper,
    _Package,
}

pub struct FFIUriPackageOrWrapperUriVariant {
    uri: Arc<FFIUri>,
}

impl FFIUriPackageOrWrapperUriVariant {
    pub fn new(uri: Arc<FFIUri>) -> FFIUriPackageOrWrapperUriVariant {
        FFIUriPackageOrWrapperUriVariant { uri }
    }

    pub fn get_uri(&self) -> Arc<FFIUri> {
        self.uri.clone()
    }
}

pub struct FFIUriPackageOrWrapperWrapperVariant {
    uri: Arc<FFIUri>,
    wrapper: Arc<ExtWrapper>,
}

impl FFIUriPackageOrWrapperWrapperVariant {
    pub fn new(uri: Arc<FFIUri>, wrapper: Box<dyn FFIWrapper>) -> FFIUriPackageOrWrapperWrapperVariant {
        FFIUriPackageOrWrapperWrapperVariant { uri, wrapper: Arc::new(ExtWrapper(wrapper)) }
    }

    pub fn get_uri(&self) -> Arc<FFIUri> {
        self.uri.clone()
    }

    pub fn get_wrapper(&self) -> Arc<ExtWrapper> {
        self.wrapper.clone()
    }
}

#[derive(Clone)]
pub struct FFIUriPackageOrWrapperPackageVariant {
    uri: Arc<FFIUri>,
    package: Arc<FFIWrapPackage>,
}

impl FFIUriPackageOrWrapperPackageVariant {
    pub fn new(uri: Arc<FFIUri>, package: Arc<FFIWrapPackage>) -> FFIUriPackageOrWrapperPackageVariant {
        FFIUriPackageOrWrapperPackageVariant { uri, package }
    }

    pub fn get_uri(&self) -> Arc<FFIUri> {
        self.uri.clone()
    }

    pub fn get_package(&self) -> Arc<FFIWrapPackage> {
        self.package.clone()
    }
}

#[derive(Clone)]
pub struct FFIUriPackageOrWrapper {
    kind: FFIUriPackageOrWrapperKind,
    uri: Option<Arc<FFIUriPackageOrWrapperUriVariant>>,
    wrapper: Option<Arc<FFIUriPackageOrWrapperWrapperVariant>>,
    package: Option<Arc<FFIUriPackageOrWrapperPackageVariant>>,
}

impl FFIUriPackageOrWrapper {
    pub fn new_uri(uri: Arc<FFIUri>) -> FFIUriPackageOrWrapper {
        Self {
            kind: FFIUriPackageOrWrapperKind::_Uri,
            uri: Some(Arc::new(FFIUriPackageOrWrapperUriVariant::new(uri))),
            wrapper: None,
            package: None,
        }
    }

    pub fn new_wrapper(uri: Arc<FFIUri>, wrapper: Box<dyn FFIWrapper>) -> FFIUriPackageOrWrapper {
        Self {
            kind: FFIUriPackageOrWrapperKind::_Wrapper,
            uri: None,
            wrapper: Some(Arc::new(FFIUriPackageOrWrapperWrapperVariant::new(uri, wrapper))),
            package: None,
        }
    }

    pub fn new_package(uri: Arc<FFIUri>, package: Arc<FFIWrapPackage>) -> FFIUriPackageOrWrapper {
        Self {
            kind: FFIUriPackageOrWrapperKind::_Package,
            uri: None,
            wrapper: None,
            package: Some(Arc::new(FFIUriPackageOrWrapperPackageVariant::new(uri, package))),
        }
    }

    pub fn get_kind(&self) -> FFIUriPackageOrWrapperKind {
        self.kind.clone()
    }

    pub fn get_uri(&self) -> Option<Arc<FFIUriPackageOrWrapperUriVariant>> {
        self.uri.clone()
    }

    pub fn get_wrapper(&self) -> Option<Arc<FFIUriPackageOrWrapperWrapperVariant>> {
        self.wrapper.clone()
    }

    pub fn get_package(&self) -> Option<Arc<FFIUriPackageOrWrapperPackageVariant>> {
        self.package.clone()
    }
}

impl From<FFIUriPackageOrWrapper> for UriPackageOrWrapper {
    fn from(value: FFIUriPackageOrWrapper) -> Self {
        match value.get_kind() {
            FFIUriPackageOrWrapperKind::_Uri => {
                let variant = value.get_uri().unwrap();
                UriPackageOrWrapper::Uri(variant.get_uri().0.clone())
            }
            FFIUriPackageOrWrapperKind::_Wrapper => {
                let variant = value.get_wrapper().unwrap();
                UriPackageOrWrapper::Wrapper(
                    variant.get_uri().0.clone(),
                    variant.get_wrapper().clone(),
                )
            }
            FFIUriPackageOrWrapperKind::_Package => {
                let variant = value.get_package().unwrap();
                UriPackageOrWrapper::Package(
                    variant.get_uri().0.clone(),
                    variant.get_package().0.clone(),
                )
            }
        }
    }
}
