use std::sync::{Arc};

use polywrap_client::core::{package::WrapPackage};

pub struct FFIWrapPackage(pub Arc<dyn WrapPackage>);

impl FFIWrapPackage {
  pub fn new(package: Arc<dyn WrapPackage>) -> FFIWrapPackage {
    FFIWrapPackage(package)
  }
}