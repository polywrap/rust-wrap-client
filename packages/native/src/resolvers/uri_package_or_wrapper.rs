use std::{
    sync::{Arc, Mutex},
};

use polywrap_client::core::{
    package::WrapPackage,
    resolvers::{uri_resolution_context::UriPackageOrWrapper},
    wrapper::Wrapper,
};

pub enum FFIUriPackageOrWrapper {
    Uri { uri: String },
    Wrapper { uri: String, wrapper: Box<dyn Wrapper> },
    Package { uri: String, package: Box<dyn WrapPackage> },
}

impl From<FFIUriPackageOrWrapper> for UriPackageOrWrapper {
    fn from(value: FFIUriPackageOrWrapper) -> Self {
        match value {
          FFIUriPackageOrWrapper::Uri { uri } => {
                UriPackageOrWrapper::Uri(uri.try_into().unwrap())
            }
            FFIUriPackageOrWrapper::Wrapper { uri, wrapper } => {
              UriPackageOrWrapper::Wrapper(uri.try_into().unwrap(), Arc::new(Mutex::new(wrapper)))
            }
            FFIUriPackageOrWrapper::Package { uri, package } => {
              UriPackageOrWrapper::Package(uri.try_into().unwrap(), Arc::new(Mutex::new(package)))
            }
        }
    }
}

