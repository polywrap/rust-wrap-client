use crate::utils::{Buffer, get_string_from_cstr_ptr, into_raw_ptr_and_forget};

#[no_mangle]
pub extern "C" fn encode(json_str: *const std::ffi::c_char) -> *const Buffer {
  let json_str = get_string_from_cstr_ptr(json_str);
  let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();
  let buffer: Buffer = polywrap_client::msgpack::serialize(json).unwrap().into();

  into_raw_ptr_and_forget(buffer) as *const _
}