use std::{fmt::Display, sync::Arc};

use polywrap_client::core::resolution::uri_resolution_context::UriPackageOrWrapper;

use crate::{
    error::FFIError,
    package::{FFIWrapPackage, WrapPackageWrapping},
    uri::FFIUri,
    wrapper::{FFIWrapper, WrapperWrapping},
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

pub struct FFIUriWrapper(Arc<FFIUri>, Box<dyn FFIWrapper>);

impl FFIUriWrapper {
    pub fn get_uri(&self) -> Arc<FFIUri> {
      self.0
    }

    pub fn get_wrapper(&self) -> Box<dyn FFIWrapper> {
      self.1
    }
}

pub struct FFIUriWrapPackage(Arc<FFIUri>, Box<dyn FFIWrapPackage>);

impl FFIUriWrapPackage {
    pub fn get_uri(&self) -> Arc<FFIUri> {
      self.0
    }

    pub fn get_package(&self) -> Box<dyn FFIWrapPackage> {
      self.1
    }
}

pub struct FFIUriPackageOrWrapper(pub UriPackageOrWrapper);

impl FFIUriPackageOrWrapper {
    pub fn from_uri(uri: Arc<FFIUri>) -> Self {
        let uri_package_or_wrapper = UriPackageOrWrapper::Uri(uri.0);

        Self(uri_package_or_wrapper)
    }

    pub fn from_package(uri: Arc<FFIUri>, package: Box<dyn FFIWrapPackage>) -> Self {
        let uri_package_or_wrapper =
            UriPackageOrWrapper::Package(uri.0, Arc::new(WrapPackageWrapping(package)));

        Self(uri_package_or_wrapper)
    }

    pub fn from_wrapper(uri: Arc<FFIUri>, wrapper: Box<dyn FFIWrapper>) -> Self {
        let uri_package_or_wrapper =
            UriPackageOrWrapper::Wrapper(uri.0, Arc::new(WrapperWrapping(wrapper)));

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

        match self.0 {
          UriPackageOrWrapper::Uri(uri) => todo!(),
          _ => Err(FFIError::ResolutionError {
            err: format!(
                "Cannot cast uri_package_or_wrapper as Package. This instance is of type '{}'",
                kind
              ),
          }),
        }
    }

    pub fn as_wrapper(&self) -> Result<Arc<FFIUriWrapper>, FFIError> {
        let kind = self.get_kind();

        match self.0 {
          UriPackageOrWrapper::Wrapper(uri, wrapper) => {
            Ok(Arc::new(FFIUriWrapper(
              Arc::new(FFIUri(uri)),
              wrapper
            )))
          },
          _ => Err(FFIError::ResolutionError {
            err: format!(
                "Cannot cast uri_package_or_wrapper as Package. This instance is of type '{}'",
                kind
              ),
          }),
        }
    }

    pub fn as_package(&self) -> Result<Arc<FFIUriWrapPackage>, FFIError> {
        let kind = self.get_kind();

        match self.0 {
          UriPackageOrWrapper::Package(uri, package) => {
            Ok(Arc::new(FFIUriWrapPackage(
              Arc::new(FFIUri(uri)),
              package
            )))
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
