pub use polywrap_client as client;
pub use polywrap_client_builder as builder;
pub use polywrap_client_default_config as default_config;
pub use polywrap_core as core;
pub use polywrap_msgpack_serde as msgpack;
pub use polywrap_plugin as plugin;
pub use polywrap_resolvers as resolvers;
pub use polywrap_wasm as wasm;
pub use wrap_manifest_schemas as wrap_manifest;
pub use polywrap_resolver_extensions as resolver_extensions;

pub use builder::*;
pub use client::*;
pub use default_config::*;
pub use plugin::*;
pub use resolvers::*;
pub use wasm::*;
pub use resolver_extensions::*;

pub use crate::core::error::Error;

// Serde JSON also has a to_vec and from slice method so this makes
// the msgpack to_vec and from_slice the one exported by default
pub use msgpack::{to_vec, from_slice};
pub use serde::*;
pub use serde_json::*;