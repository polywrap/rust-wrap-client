use polywrap_client::{builder::types::{BuilderConfig, ClientConfigHandler}, client::PolywrapClient, core::invoke::Invoker, msgpack::serialize};
use serde_json::Value;
use crate::utils::{instantiate_from_ptr, into_raw_ptr_and_forget, Buffer, raw_ptr_from_str};
use std::{ffi::c_char, slice::from_raw_parts};
use polywrap_client::{core::{uri::Uri}};
use crate::utils::{get_string_from_cstr_ptr};

#[no_mangle]
pub extern "C" fn create_client(builder_config_ptr: *mut BuilderConfig) -> *mut PolywrapClient {
  let builder = instantiate_from_ptr(builder_config_ptr);
  let config = builder.build();

  let client = PolywrapClient::new(config);
  into_raw_ptr_and_forget(client) as *mut PolywrapClient
}

#[no_mangle]
pub extern "C" fn invoke_raw(
  client_ptr: *mut PolywrapClient,
  uri: *const c_char,
  method: *const c_char,
  args: *const c_char,
  env: *const c_char,
) -> *const c_char {
  let client = unsafe { &*client_ptr };
  let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();
  let method = get_string_from_cstr_ptr(method);
  let args = get_string_from_cstr_ptr(args);

  let env = if !env.is_null() {
    Some(serde_json::from_str(&get_string_from_cstr_ptr(env)).unwrap())
  } else {
    None
  };

  let args: Value = serde_json::from_str(&args).unwrap();
  let args = serialize(args).unwrap();

  let result = client.invoke_raw(&uri, &method, Some(&args), env, None).unwrap();
  let result_json: serde_json::Value = polywrap_client::msgpack::decode(&result).unwrap();
  let result_json = result_json.to_string();

  raw_ptr_from_str(&result_json)
}