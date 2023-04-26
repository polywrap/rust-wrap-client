use std::sync::Arc;

use polywrap_client::core::{resolvers::uri_resolution_context::UriPackageOrWrapper, uri::Uri};

use crate::{package::FFIWrapPackage, wrapper::FFIWrapper};

#[derive(Clone)]
pub enum FFIUriPackageOrWrapperKind {
    Uri,
    Wrapper,
    Package,
}

pub struct UriVariant {
    uri: Arc<Uri>,
}

impl UriVariant {
    pub fn new(uri: Arc<Uri>) -> UriVariant {
        UriVariant { uri }
    }

    pub fn get_uri(&self) -> Arc<Uri> {
        self.uri.clone()
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

    pub fn get_uri(&self) -> Arc<Uri> {
        self.uri.clone()
    }

    pub fn get_wrapper(&self) -> Arc<FFIWrapper> {
        self.wrapper.clone()
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

    pub fn get_uri(&self) -> Arc<Uri> {
        self.uri.clone()
    }

    pub fn get_package(&self) -> Arc<FFIWrapPackage> {
        self.package.clone()
    }
}

pub struct FFIUriPackageOrWrapper {
    kind: FFIUriPackageOrWrapperKind,
    uri: Option<Arc<UriVariant>>,
    wrapper: Option<Arc<WrapperVariant>>,
    package: Option<Arc<PackageVariant>>,
}

impl FFIUriPackageOrWrapper {
    pub fn new_uri(uri: Arc<Uri>) -> FFIUriPackageOrWrapper {
        Self {
            kind: FFIUriPackageOrWrapperKind::Uri,
            uri: Some(Arc::new(UriVariant::new(uri))),
            wrapper: None,
            package: None,
        }
    }

    pub fn new_wrapper(uri: Arc<Uri>, wrapper: Arc<FFIWrapper>) -> FFIUriPackageOrWrapper {
        Self {
            kind: FFIUriPackageOrWrapperKind::Wrapper,
            uri: None,
            wrapper: Some(Arc::new(WrapperVariant::new(uri, wrapper))),
            package: None,
        }
    }

    pub fn new_package(uri: Arc<Uri>, package: Arc<FFIWrapPackage>) -> FFIUriPackageOrWrapper {
        Self {
            kind: FFIUriPackageOrWrapperKind::Package,
            uri: None,
            wrapper: None,
            package: Some(Arc::new(PackageVariant::new(uri, package))),
        }
    }

    pub fn get_kind(&self) -> FFIUriPackageOrWrapperKind {
        self.kind.clone()
    }

    pub fn get_uri(&self) -> Option<Arc<UriVariant>> {
        self.uri.clone()
    }

    pub fn get_wrapper(&self) -> Option<Arc<WrapperVariant>> {
        self.wrapper.clone()
    }

    pub fn get_package(&self) -> Option<Arc<PackageVariant>> {
        self.package.clone()
    }
}

impl From<FFIUriPackageOrWrapper> for UriPackageOrWrapper {
    fn from(value: FFIUriPackageOrWrapper) -> Self {
        match value.get_kind() {
            FFIUriPackageOrWrapperKind::Uri => {
                let variant = value.get_uri().unwrap();
                UriPackageOrWrapper::Uri(variant.get_uri().as_ref().clone())
            }
            FFIUriPackageOrWrapperKind::Wrapper => {
                let variant = value.get_wrapper().unwrap();
                UriPackageOrWrapper::Wrapper(
                    variant.get_uri().as_ref().clone(),
                    variant.get_wrapper().0.clone(),
                )
            }
            FFIUriPackageOrWrapperKind::Package => {
                let variant = value.get_package().unwrap();
                UriPackageOrWrapper::Package(
                    variant.get_uri().as_ref().clone(),
                    variant.get_package().0.clone(),
                )
            }
        }
    }
}
