use std::sync::{Arc, Mutex};

use polywrap_client::core::{package::WrapPackage};

pub struct FFIWrapPackage(pub Arc<Mutex<Box<dyn WrapPackage>>>);

impl FFIWrapPackage {
  pub fn new(package: Arc<Mutex<Box<dyn WrapPackage>>>) -> FFIWrapPackage {
    FFIWrapPackage(package)
  }
}