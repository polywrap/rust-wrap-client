use std::{fmt::Debug, sync::Arc};

use crate::{
    error::FFIError,
    wrapper::{IFFIWrapper, FFIWrapper},
};
use polywrap_client::core::{error::Error, package::WrapPackage, wrapper::Wrapper};

pub trait IFFIWrapPackage: Debug + Send + Sync {
    fn ffi_create_wrapper(&self) -> Result<Box<dyn IFFIWrapper>, FFIError>;
}

impl IFFIWrapPackage for Arc<dyn WrapPackage> {
    fn ffi_create_wrapper(&self) -> Result<Box<dyn IFFIWrapper>, FFIError> {
        let arc_self = self.clone();
        let wrapper = WrapPackage::create_wrapper(arc_self.as_ref())?;
        Ok(Box::new(wrapper))
    }
}

#[derive(Debug)]
pub struct FFIWrapPackage(pub Box<dyn IFFIWrapPackage>);

impl FFIWrapPackage {
  pub fn new(wrap_package: Box<dyn IFFIWrapPackage>) -> Self {
    Self(wrap_package)
  }

  pub fn create_wrapper(&self) -> Result<Arc<FFIWrapper>, FFIError> {
    let wrapper = self.0.ffi_create_wrapper()?;
    Ok(Arc::new(FFIWrapper(wrapper)))
  }
}

impl WrapPackage for FFIWrapPackage {
    fn create_wrapper(&self) -> Result<Arc<dyn Wrapper>, Error> {
        let ffi_wrapper = self.0.ffi_create_wrapper()?;
        Ok(Arc::new(FFIWrapper(ffi_wrapper)))
    }

    fn get_manifest(
        &self,
        _: Option<&polywrap_client::core::package::GetManifestOptions>,
    ) -> Result<polywrap_client::wrap_manifest::versions::WrapManifest, Error> {
        unimplemented!("get_manifest is not implemented for IFFIWrapPackage")
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use polywrap_msgpack_serde::from_slice;
    use polywrap_tests_utils::mocks::{get_mock_invoker, get_mock_package};

    use crate::invoker::FFIInvoker;

    use super::IFFIWrapPackage;

    fn get_mocks() -> (Box<dyn IFFIWrapPackage>, FFIInvoker) {
        (Box::new(get_mock_package()), FFIInvoker(get_mock_invoker()))
    }

    #[test]
    fn test_ffi_package() {
        let (ffi_package, ffi_invoker) = get_mocks();
        let ffi_wrapper = ffi_package.ffi_create_wrapper().unwrap();
        let response =
            ffi_wrapper.ffi_invoke("foo".to_string(), None, None, Arc::new(ffi_invoker));
        assert!(from_slice::<bool>(&response.unwrap()).unwrap());
    }
}
