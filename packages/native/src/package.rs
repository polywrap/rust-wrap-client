use std::{fmt::Debug, sync::Arc};

use polywrap_client::core::{error::Error, package::WrapPackage, wrapper::Wrapper};
use crate::wrapper::{FFIWrapper, ExtWrapper};

pub trait FFIWrapPackage: Debug + Send + Sync {
    fn create_wrapper(
      &self
    ) -> Box<dyn FFIWrapper>;
}

impl FFIWrapPackage for Arc<dyn WrapPackage> {
    fn create_wrapper(
      &self
    ) -> Box<dyn FFIWrapper> {
      let arc_self = self.clone();
      let wrapper = WrapPackage::create_wrapper(arc_self.as_ref()).unwrap();
      Box::new(wrapper)
    }
}

#[derive(Debug)]
pub struct ExtWrapPackage(pub Box<dyn FFIWrapPackage>);

impl WrapPackage for ExtWrapPackage {
  fn create_wrapper(&self) -> Result<Arc<dyn Wrapper>, Error> {
    let ffi_wrapper = self.0.create_wrapper();
    Ok(Arc::new(ExtWrapper(ffi_wrapper)))
  }

  fn get_manifest(
          &self,
          _: Option<&polywrap_client::core::package::GetManifestOptions>,
      ) -> Result<polywrap_client::wrap_manifest::versions::WrapManifest, Error> {
      unimplemented!("get_manifest is not implemented for FFIWrapPackage")
  }
}