pub mod builder;
pub mod resolvers;
pub mod invoker;
pub mod wasm_wrapper;
pub mod client;
pub mod wrapper;
pub mod package;
pub mod uri;

use builder::*;
use invoker::*;
use wasm_wrapper::*;
use client::*;
use wrapper::*;
use package::*;
use uri::*;
use resolvers::{
  _static::*,
  extendable::*,
  ffi_resolver::*,
  recursive::*,
  uri_package_or_wrapper::*
};

use polywrap_client::core::error::Error;

uniffi::include_scaffolding!("main");