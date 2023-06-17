use std::{fmt::Debug, sync::Arc};

use crate::{
    error::FFIError,
    wrapper::{FFIWrapper, WrapperWrapping},
};
use polywrap_client::core::{error::Error, package::WrapPackage, wrapper::Wrapper};

pub trait FFIWrapPackage: Debug + Send + Sync {
    fn create_wrapper(&self) -> Result<Box<dyn FFIWrapper>, FFIError>;
}

impl FFIWrapPackage for Arc<dyn WrapPackage> {
    fn create_wrapper(&self) -> Result<Box<dyn FFIWrapper>, FFIError> {
        let arc_self = self.clone();
        let wrapper = WrapPackage::create_wrapper(arc_self.as_ref())?;
        Ok(Box::new(wrapper))
    }
}

#[derive(Debug)]
pub struct WrapPackageWrapping(pub Box<dyn FFIWrapPackage>);

impl WrapPackage for WrapPackageWrapping {
    fn create_wrapper(&self) -> Result<Arc<dyn Wrapper>, Error> {
        let ffi_wrapper = self.0.create_wrapper()?;
        Ok(Arc::new(WrapperWrapping(ffi_wrapper)))
    }

    fn get_manifest(
        &self,
        _: Option<&polywrap_client::core::package::GetManifestOptions>,
    ) -> Result<polywrap_client::wrap_manifest::versions::WrapManifest, Error> {
        unimplemented!("get_manifest is not implemented for FFIWrapPackage")
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use polywrap_client::msgpack::decode;
    use polywrap_tests_utils::mocks::{get_mock_invoker, get_mock_package};

    use crate::invoker::FFIInvoker;

    use super::FFIWrapPackage;

    fn get_mocks() -> (Box<dyn FFIWrapPackage>, FFIInvoker) {
        (Box::new(get_mock_package()), FFIInvoker(get_mock_invoker()))
    }

    #[test]
    fn test_ffi_package() {
        let (ffi_package, ffi_invoker) = get_mocks();
        let ffi_wrapper = ffi_package.create_wrapper().unwrap();
        let response =
            ffi_wrapper.invoke("foo".to_string(), None, None, Arc::new(ffi_invoker), None);
        assert!(decode::<bool>(&response.unwrap()).unwrap());
    }
}
