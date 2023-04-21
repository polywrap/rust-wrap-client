use std::{
    sync::{Arc, Mutex},
};

use polywrap_client::core::{
    package::WrapPackage,
    resolvers::{uri_resolution_context::UriPackageOrWrapper as InnerUriPackageOrWrapper},
    wrapper::Wrapper,
};

pub enum UriPackageOrWrapper {
    Uri { uri: String },
    Wrapper { uri: String, wrapper: Box<dyn Wrapper> },
    Package { uri: String, package: Box<dyn WrapPackage> },
}

impl From<UriPackageOrWrapper> for InnerUriPackageOrWrapper {
    fn from(value: UriPackageOrWrapper) -> Self {
        match value {
            UriPackageOrWrapper::Uri { uri } => {
                InnerUriPackageOrWrapper::Uri(uri.try_into().unwrap())
            }
            UriPackageOrWrapper::Wrapper { uri, wrapper } => {
              InnerUriPackageOrWrapper::Wrapper(uri.try_into().unwrap(), Arc::new(Mutex::new(wrapper)))
            }
            UriPackageOrWrapper::Package { uri, package } => {
              InnerUriPackageOrWrapper::Package(uri.try_into().unwrap(), Arc::new(Mutex::new(package)))
            }
        }
    }
}

