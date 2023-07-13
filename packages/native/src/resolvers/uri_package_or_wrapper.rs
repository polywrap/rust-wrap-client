use std::{fmt::Display, sync::Arc};

use polywrap_client::core::{
    package::WrapPackage, resolution::uri_resolution_context::UriPackageOrWrapper, uri::Uri,
    wrapper::Wrapper,
};

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

pub trait FFIUriWrapper {
    fn get_uri(&self) -> Arc<FFIUri>;
    fn get_wrapper(&self) -> Box<dyn FFIWrapper>;
}

pub trait FFIUriWrapPackage {
    fn get_uri(&self) -> Arc<FFIUri>;
    fn get_package(&self) -> Box<dyn FFIWrapPackage>;
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

    pub fn as_wrapper(&self) -> Result<Box<dyn FFIUriWrapper>, FFIError> {
        let kind = self.get_kind();

        match self.0 {
          UriPackageOrWrapper::Wrapper(uri, wrapper) => todo!(),
          _ => Err(FFIError::ResolutionError {
            err: format!(
                "Cannot cast uri_package_or_wrapper as Package. This instance is of type '{}'",
                kind
              ),
          }),
        }
    }

    pub fn as_package(&self) -> Result<Box<dyn FFIUriWrapPackage>, FFIError> {
        let kind = self.get_kind();

        match self.0 {
          UriPackageOrWrapper::Package(uri, package) => todo!(),
          _ => Err(FFIError::ResolutionError {
            err: format!(
                "Cannot cast uri_package_or_wrapper as Package. This instance is of type '{}'",
                kind
              ),
          }),
        }
    }
}

// impl From<FFIUriPackageOrWrapper> for UriPackageOrWrapper {
//     fn from(value: FFIUriPackageOrWrapper) -> Self {
//         match value.as_ref().get_kind() {
//             FFIUriPackageOrWrapperKind::URI => UriPackageOrWrapper::Uri(value.as_uri().0.clone()),
//             FFIUriPackageOrWrapperKind::WRAPPER => {
//                 let uri_wrapper = value.as_wrapper();
//                 let uri = uri_wrapper.as_ref().get_uri();
//                 let wrapper = uri_wrapper.as_ref().get_wrapper();

//                 UriPackageOrWrapper::Wrapper(uri.0.clone(), Arc::new(WrapperWrapping(wrapper)))
//             }
//             FFIUriPackageOrWrapperKind::PACKAGE => {
//                 let uri_package = value.as_package();
//                 let uri = uri_package.as_ref().get_uri();
//                 let package = uri_package.as_ref().get_package();

//                 UriPackageOrWrapper::Package(uri.0.clone(), Arc::new(WrapPackageWrapping(package)))
//             }
//         }
//     }
// }

// impl FFIUriPackageOrWrapper for UriPackageOrWrapper {
//     fn get_kind(&self) -> FFIUriPackageOrWrapperKind {
//         match self {
//             UriPackageOrWrapper::Uri(_) => FFIUriPackageOrWrapperKind::URI,
//             UriPackageOrWrapper::Wrapper(_, _) => FFIUriPackageOrWrapperKind::WRAPPER,
//             UriPackageOrWrapper::Package(_, _) => FFIUriPackageOrWrapperKind::PACKAGE,
//         }
//     }

//     fn as_uri(&self) -> Arc<FFIUri> {
//         match self {
//             UriPackageOrWrapper::Uri(uri) => Arc::new(FFIUri(uri.clone())),
//             _ => panic!("Cannot cast this instance of UriPackageOrWrapper as Uri"),
//         }
//     }

//     fn as_wrapper(&self) -> Box<dyn FFIUriWrapper> {
//         match self {
//             UriPackageOrWrapper::Wrapper(uri, wrapper) => {
//                 Box::new(UriWrapper(uri.clone(), wrapper.clone()))
//             }
//             _ => panic!("Cannot cast this instance of UriPackageOrWrapper as Wrapper"),
//         }
//     }

//     fn as_package(&self) -> Box<dyn FFIUriWrapPackage> {
//         match self {
//             UriPackageOrWrapper::Package(uri, package) => {
//                 Box::new(UriWrapPackage(uri.clone(), package.clone()))
//             }
//             _ => panic!("Cannot cast this instance of UriPackageOrWrapper as WrapPackage"),
//         }
//     }
// }

pub struct UriWrapper(Uri, Arc<dyn Wrapper>);

impl FFIUriWrapper for UriWrapper {
    fn get_uri(&self) -> Arc<FFIUri> {
        Arc::new(FFIUri(self.0.clone()))
    }

    fn get_wrapper(&self) -> Box<dyn FFIWrapper> {
        Box::new(self.1.clone())
    }
}

pub struct UriWrapPackage(Uri, Arc<dyn WrapPackage>);

impl FFIUriWrapPackage for UriWrapPackage {
    fn get_uri(&self) -> Arc<FFIUri> {
        Arc::new(FFIUri(self.0.clone()))
    }

    fn get_package(&self) -> Box<dyn FFIWrapPackage> {
        Box::new(self.1.clone())
    }
}
