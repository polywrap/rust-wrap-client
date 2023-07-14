use std::{fmt::Display, sync::Arc};

use polywrap_client::core::resolution::uri_resolution_context::UriPackageOrWrapper;

use crate::{
    error::FFIError,
    package::{IFFIWrapPackage, WrapPackageWrapping},
    uri::FFIUri,
    wrapper::{IFFIWrapper, WrapperWrapping},
};

#[derive(Debug)]
pub enum FFIUriPackageOrWrapperKind {
    URI,
    PACKAGE,
    WRAPPER,
}

impl Display for FFIUriPackageOrWrapperKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printable = match &self {
            FFIUriPackageOrWrapperKind::URI => "Uri",
            FFIUriPackageOrWrapperKind::PACKAGE => "Package",
            FFIUriPackageOrWrapperKind::WRAPPER => "Wrapper",
        };

        write!(f, "{}", printable)
    }
}

pub struct FFIUriWrapper {
  pub uri: Arc<FFIUri>,
  pub wrapper: Box<dyn IFFIWrapper>
}

impl FFIUriWrapper {
  pub fn new(uri: Arc<FFIUri>, wrapper: Box<dyn IFFIWrapper>) -> Self {
    FFIUriWrapper { uri, wrapper }
  }
}

pub struct FFIUriWrapPackage {
  pub uri: Arc<FFIUri>,
  pub package: Box<dyn IFFIWrapPackage>
}

impl FFIUriWrapPackage {
  pub fn new(uri: Arc<FFIUri>, package: Box<dyn IFFIWrapPackage>) -> Self {
    FFIUriWrapPackage { uri, package }
  }
}

pub struct FFIUriPackageOrWrapper(pub UriPackageOrWrapper);

impl FFIUriPackageOrWrapper {
    pub fn from_uri(uri: Arc<FFIUri>) -> Self {
        let uri_package_or_wrapper = UriPackageOrWrapper::Uri(uri.as_ref().0.clone());

        Self(uri_package_or_wrapper)
    }

    pub fn from_package(uri: Arc<FFIUri>, package: Box<dyn IFFIWrapPackage>) -> Self {
        let uri_package_or_wrapper =
            UriPackageOrWrapper::Package(uri.as_ref().0.clone(), Arc::new(WrapPackageWrapping(package)));

        Self(uri_package_or_wrapper)
    }

    pub fn from_wrapper(uri: Arc<FFIUri>, wrapper: Box<dyn IFFIWrapper>) -> Self {
        let uri_package_or_wrapper =
            UriPackageOrWrapper::Wrapper(uri.as_ref().0.clone(), Arc::new(WrapperWrapping(wrapper)));

        Self(uri_package_or_wrapper)
    }

    pub fn get_kind(&self) -> FFIUriPackageOrWrapperKind {
        match self.0 {
            UriPackageOrWrapper::Uri(_) => FFIUriPackageOrWrapperKind::URI,
            UriPackageOrWrapper::Wrapper(_, _) => FFIUriPackageOrWrapperKind::WRAPPER,
            UriPackageOrWrapper::Package(_, _) => FFIUriPackageOrWrapperKind::PACKAGE,
        }
    }

    pub fn as_uri(&self) -> Result<Arc<FFIUri>, FFIError> {
        let kind = self.get_kind();

        match self.0.clone() {
          UriPackageOrWrapper::Uri(uri) => Ok(Arc::new(FFIUri(uri))),
          _ => Err(FFIError::ResolutionError {
            err: format!(
                "Cannot cast uri_package_or_wrapper as Package. This instance is of type '{}'",
                kind
              ),
          }),
        }
    }

    pub fn as_wrapper(&self) -> Result<FFIUriWrapper, FFIError> {
        let kind = self.get_kind();

        match self.0.clone() {
          UriPackageOrWrapper::Wrapper(uri, wrapper) => {
            Ok(FFIUriWrapper::new(
              Arc::new(FFIUri(uri)),
              Box::new(wrapper)
            ))
          },
          _ => Err(FFIError::ResolutionError {
            err: format!(
                "Cannot cast uri_package_or_wrapper as Package. This instance is of type '{}'",
                kind
              ),
          }),
        }
    }

    pub fn as_package(&self) -> Result<FFIUriWrapPackage, FFIError> {
        let kind = self.get_kind();

        match self.0.clone() {
          UriPackageOrWrapper::Package(uri, package) => {
            Ok(FFIUriWrapPackage::new(
              Arc::new(FFIUri(uri)),
              Box::new(package)
            ))
          },
          _ => Err(FFIError::ResolutionError {
            err: format!(
                "Cannot cast uri_package_or_wrapper as Package. This instance is of type '{}'",
                kind
              ),
          }),
        }
    }
}
