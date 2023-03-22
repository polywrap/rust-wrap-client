use polywrap_client::{builder::types::{BuilderConfig, ClientConfigHandler}, client::PolywrapClient, core::invoke::Invoker};
use crate::utils::{instantiate_from_ptr, into_raw_ptr_and_forget, Buffer};
use std::ffi::c_char;
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
  args: *const Buffer,
  env: *const c_char,
) -> *const Buffer {
  let client = instantiate_from_ptr(client_ptr);
  let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();
  let method = get_string_from_cstr_ptr(method);
  let mut _args_buffer: Option<Vec<u8>> = None;
  let args = if !args.is_null() {
    let buffer: Vec<u8> = instantiate_from_ptr(args as *mut Buffer).into();
    _args_buffer = Some(buffer);
    _args_buffer.as_deref()
  } else {
    None
  };

  let env = if !env.is_null() {
    Some(serde_json::from_str(&get_string_from_cstr_ptr(env)).unwrap())
  } else {
    None
  };

  let result = client.invoke_raw(&uri, &method, args, env, None).unwrap();
  let result_buffer = Buffer {
    data: result.as_ptr() as *mut u8,
    len: result.len()
  };

  into_raw_ptr_and_forget(result_buffer) as *const Buffer
}