pub mod builder;
pub mod resolvers;
pub mod invoker;
pub mod wasm_wrapper;
pub mod client;
pub mod wrapper;
pub mod package;
pub mod uri;
pub mod error;

use builder::*;
use invoker::*;
use wasm_wrapper::*;
use client::*;
use wrapper::*;
use package::*;
use uri::*;
use resolvers::{
  _static::*,
  ffi_resolver::*,
  recursive::*,
  uri_package_or_wrapper::*,
  resolution_context::*
};
use error::*;

uniffi::include_scaffolding!("main");