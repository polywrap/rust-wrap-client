use std::sync::Arc;

use polywrap_client::core::{
    resolvers::{uri_resolution_context::UriPackageOrWrapper}, uri::Uri,
};

use crate::{wrapper::FFIWrapper, package::FFIWrapPackage};

pub enum FFIUriPackageOrWrapper {
    Uri { uri: Arc<Uri> },
    Wrapper { uri: Arc<Uri>, wrapper: Arc<FFIWrapper> },
    Package { uri: Arc<Uri>, package: Arc<FFIWrapPackage> },
}

impl From<FFIUriPackageOrWrapper> for UriPackageOrWrapper {
    fn from(value: FFIUriPackageOrWrapper) -> Self {
        match value {
          FFIUriPackageOrWrapper::Uri { uri } => {
                UriPackageOrWrapper::Uri(uri.as_ref().clone())
            }
            FFIUriPackageOrWrapper::Wrapper { uri, wrapper } => {
              UriPackageOrWrapper::Wrapper(uri.as_ref().clone(), wrapper.0.clone())
            }
            FFIUriPackageOrWrapper::Package { uri, package } => {
              UriPackageOrWrapper::Package(uri.as_ref().clone(), package.0.clone())
            }
        }
    }
}

