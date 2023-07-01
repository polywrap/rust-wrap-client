use std::sync::Arc;

use polywrap_client::core::{
    package::WrapPackage, resolution::uri_resolution_context::UriPackageOrWrapper, uri::Uri,
    wrapper::Wrapper,
};

use crate::{
    package::{IFFIWrapPackage, FFIWrapPackage},
    uri::FFIUri,
    wrapper::{IFFIWrapper, FFIWrapper},
};

#[derive(Debug)]
pub enum FFIUriPackageOrWrapperKind {
    URI,
    PACKAGE,
    WRAPPER,
}

pub trait IFFIUriWrapper: Send + Sync {
    fn ffi_get_uri(&self) -> Arc<FFIUri>;
    fn ffi_get_wrapper(&self) -> Box<dyn IFFIWrapper>;
}

pub struct FFIUriWrapper(pub Box<dyn IFFIUriWrapper>);

impl FFIUriWrapper {
  pub fn new(uri_wrapper: Box<dyn IFFIUriWrapper>) -> Self {
    Self(uri_wrapper)
  }

  pub fn get_uri(&self) -> Arc<FFIUri> {
    self.0.ffi_get_uri()
  }
  
  pub fn get_wrapper(&self) -> Arc<FFIWrapper> {
    let wrapper = self.0.ffi_get_wrapper();
    Arc::new(FFIWrapper(wrapper))
  }
}

pub trait IFFIUriWrapPackage: Send + Sync {
    fn ffi_get_uri(&self) -> Arc<FFIUri>;
    fn ffi_get_package(&self) -> Box<dyn IFFIWrapPackage>;
}

pub struct FFIUriWrapPackage(pub Box<dyn IFFIUriWrapPackage>);

impl FFIUriWrapPackage {
  pub fn new(uri_wrap_package: Box<dyn IFFIUriWrapPackage>) -> Self {
    Self(uri_wrap_package)
  }

  pub fn get_uri(&self) -> Arc<FFIUri> {
    self.0.ffi_get_uri()
  }
  
  pub fn get_package(&self) -> Arc<FFIWrapPackage> {
    let wrapper = self.0.ffi_get_package();
    Arc::new(FFIWrapPackage(wrapper))
  }
}

pub trait IFFIUriPackageOrWrapper: Send + Sync {
    fn ffi_get_kind(&self) -> FFIUriPackageOrWrapperKind;
    fn ffi_as_uri(&self) -> Arc<FFIUri>;
    fn ffi_as_wrapper(&self) -> Box<dyn IFFIUriWrapper>;
    fn ffi_as_package(&self) -> Box<dyn IFFIUriWrapPackage>;
}

pub struct FFIUriPackageOrWrapper(pub Box<dyn IFFIUriPackageOrWrapper>);

impl FFIUriPackageOrWrapper {
  pub fn new(uri_package_or_wrapper: Box<dyn IFFIUriPackageOrWrapper>) -> Self {
    Self(uri_package_or_wrapper)
  }

  pub fn get_kind(&self) -> FFIUriPackageOrWrapperKind {
    self.0.ffi_get_kind()
  }
  
  pub fn as_uri(&self) -> Arc<FFIUri> {
    self.0.ffi_as_uri()
  }
  
  pub fn as_wrapper(&self) -> Arc<FFIUriWrapper> {
    Arc::new(FFIUriWrapper(self.0.ffi_as_wrapper()))
  }
  
  pub fn as_package(&self) -> Arc<FFIUriWrapPackage> {
    Arc::new(FFIUriWrapPackage(self.0.ffi_as_package()))
  }
}

impl From<Box<dyn IFFIUriPackageOrWrapper>> for UriPackageOrWrapper {
    fn from(value: Box<dyn IFFIUriPackageOrWrapper>) -> Self {
        match value.as_ref().ffi_get_kind() {
            FFIUriPackageOrWrapperKind::URI => UriPackageOrWrapper::Uri(value.ffi_as_uri().0.clone()),
            FFIUriPackageOrWrapperKind::WRAPPER => {
                let uri_wrapper = value.ffi_as_wrapper();
                let uri = uri_wrapper.as_ref().ffi_get_uri();
                let wrapper = uri_wrapper.as_ref().ffi_get_wrapper();

                UriPackageOrWrapper::Wrapper(uri.0.clone(), Arc::new(FFIWrapper(wrapper)))
            }
            FFIUriPackageOrWrapperKind::PACKAGE => {
                let uri_package = value.ffi_as_package();
                let uri = uri_package.as_ref().ffi_get_uri();
                let package = uri_package.as_ref().ffi_get_package();

                UriPackageOrWrapper::Package(uri.0.clone(), Arc::new(FFIWrapPackage(package)))
            }
        }
    }
}

impl IFFIUriPackageOrWrapper for UriPackageOrWrapper {
    fn ffi_get_kind(&self) -> FFIUriPackageOrWrapperKind {
        match self {
            UriPackageOrWrapper::Uri(_) => FFIUriPackageOrWrapperKind::URI,
            UriPackageOrWrapper::Wrapper(_, _) => FFIUriPackageOrWrapperKind::WRAPPER,
            UriPackageOrWrapper::Package(_, _) => FFIUriPackageOrWrapperKind::PACKAGE,
        }
    }

    fn ffi_as_uri(&self) -> Arc<FFIUri> {
        match self {
            UriPackageOrWrapper::Uri(uri) => Arc::new(FFIUri(uri.clone())),
            _ => panic!("Cannot cast this instance of UriPackageOrWrapper as Uri"),
        }
    }

    fn ffi_as_wrapper(&self) -> Box<dyn IFFIUriWrapper> {
        match self {
            UriPackageOrWrapper::Wrapper(uri, wrapper) => {
                Box::new(UriWrapper(uri.clone(), wrapper.clone()))
            }
            _ => panic!("Cannot cast this instance of UriPackageOrWrapper as Wrapper"),
        }
    }

    fn ffi_as_package(&self) -> Box<dyn IFFIUriWrapPackage> {
        match self {
            UriPackageOrWrapper::Package(uri, package) => {
                Box::new(UriWrapPackage(uri.clone(), package.clone()))
            }
            _ => panic!("Cannot cast this instance of UriPackageOrWrapper as WrapPackage"),
        }
    }
}

pub struct UriWrapper(Uri, Arc<dyn Wrapper>);

impl IFFIUriWrapper for UriWrapper {
    fn ffi_get_uri(&self) -> Arc<FFIUri> {
        Arc::new(FFIUri(self.0.clone()))
    }

    fn ffi_get_wrapper(&self) -> Box<dyn IFFIWrapper> {
        Box::new(self.1.clone())
    }
}

pub struct UriWrapPackage(Uri, Arc<dyn WrapPackage>);

impl IFFIUriWrapPackage for UriWrapPackage {
    fn ffi_get_uri(&self) -> Arc<FFIUri> {
        Arc::new(FFIUri(self.0.clone()))
    }

    fn ffi_get_package(&self) -> Box<dyn IFFIWrapPackage> {
        Box::new(self.1.clone())
    }
}
