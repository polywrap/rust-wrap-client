use std::sync::Arc;

use crate::{uri::FFIUri, wrapper::FFIWrapper, package::FFIWrapPackage};

pub enum FFIUriPackageOrWrapperKind {
  _URI,
  _PACKAGE,
  _WRAPPER,
}

pub trait FFIUriPackageOrWrapper: Send + Sync {
  fn get_kind(&self) -> Arc<FFIUriPackageOrWrapperKind>;
  fn as_uri(&self) -> Arc<FFIUri>;
  fn as_wrapper(&self) -> Box<dyn FFIWrapper>;
  fn as_package(&self) -> Box<dyn FFIWrapPackage>;
}
