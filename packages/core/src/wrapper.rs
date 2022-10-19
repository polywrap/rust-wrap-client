use std::future::Future;

use crate::invoke::{InvokeOptions, Invoker};

pub struct GetFileOptions {
  path: String,
  encoding: Option<String>,
}

pub struct GetManifestOptions {
  no_validate: Option<bool>
}

pub struct WrapManifest {}

pub trait Wrapper {
  fn invoke<I: Invoker>(&self, options: &InvokeOptions, invoker: I) -> dyn Future<Output = Result<String, String>>;
  fn get_file(&self, options: &GetFileOptions) -> dyn Future<Output = Result<String, String>>;
  fn get_manifest(&self, options: Option<&GetManifestOptions>) -> WrapManifest;
}
