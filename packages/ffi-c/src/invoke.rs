use std::ffi::c_char;

use polywrap_client::{client::PolywrapClient, core::{invoke::Invoker, uri::Uri}};

use crate::utils::{SafeOption, instantiate_from_ptr, get_string_from_cstr_ptr, into_raw_ptr_and_forget};
#[repr(C)]
pub struct ArgsBuffer {
  data: *mut u8,
  len: usize
}

#[no_mangle]
pub extern "C" fn invoke_raw(
  client_ptr: *mut PolywrapClient,
  uri: *const c_char,
  method: *const c_char,
  args: SafeOption<*const ArgsBuffer>,
  env: SafeOption<*const c_char>,
) -> *const ArgsBuffer {
  let client = instantiate_from_ptr(client_ptr);
  let uri: Uri = get_string_from_cstr_ptr(uri).try_into().unwrap();
  let method = get_string_from_cstr_ptr(method);
  let args = if let SafeOption::Some(args) = args {
    let buffer = instantiate_from_ptr(args as *mut ArgsBuffer);
    unsafe {
      let args_vec = std::slice::from_raw_parts(buffer.data, buffer.len);
      Some(args_vec)
    }
  } else {
    None
  };

  let env = match env {
    SafeOption::Some(env) => serde_json::from_str(&get_string_from_cstr_ptr(env)).unwrap(),
    SafeOption::None => None
  };

  let result = client.invoke_raw(&uri, &method, args, env, None).unwrap();
  let result_buffer = ArgsBuffer {
    data: result.as_ptr() as *mut u8,
    len: result.len()
  };

  into_raw_ptr_and_forget(result_buffer) as *const ArgsBuffer
}