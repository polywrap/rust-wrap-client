use std::{slice::from_raw_parts, sync::Arc, ffi::CString};

use polywrap_client::core::{file_reader::{SimpleFileReader, FileReader}, env::Env};
use polywrap_plugin::{module::{PluginModule, PluginWithEnv}, wrapper::PluginWrapper};
use polywrap_wasm::wasm_wrapper::WasmWrapper;

use crate::utils::{into_raw_ptr_and_forget, instantiate_from_ptr_and_take_ownership, get_string_from_cstr_ptr};

pub fn create_simple_file_reader() -> *const SimpleFileReader {
  let reader = SimpleFileReader::new();
  into_raw_ptr_and_forget(reader) as *mut SimpleFileReader
}

pub fn create_wasm_wrapper(
  wasm_module_buffer: *const u8,
  wams_module_len: usize,
  file_reader_ptr: *mut SimpleFileReader
) {
  let file_reader = Arc::from(instantiate_from_ptr_and_take_ownership(file_reader_ptr) as Box<dyn FileReader>);
  let wasm_module = unsafe {
      from_raw_parts(wasm_module_buffer, wams_module_len)
  };
  let wrapper = WasmWrapper::new(wasm_module.to_vec(), file_reader);
}

#[repr(C)]
struct ExtPluginModule {
  env: *mut std::ffi::c_char,

  _wrap_invoke: extern "C" fn(
    method_name: *const std::ffi::c_char,
    params_buffer: *const u8,
    params_len: usize,
    invoker: Arc<dyn polywrap_client::core::invoke::Invoker>,
  ) -> (*const u8, usize)
}

impl PluginWithEnv for ExtPluginModule {
    fn set_env(&mut self, env: polywrap_client::core::env::Env) {
        let stringified_env = env.to_string();
        let stringified_env = CString::new(stringified_env).unwrap();
        let env_string_ptr = stringified_env.into_raw();
        
        self.env = env_string_ptr
    }

    fn get_env(&self, key: String) -> Option<&polywrap_client::core::env::Env> {
        let env = get_string_from_cstr_ptr(self.env);
        let env = serde_json::from_str::<serde_json::Value>(&env).unwrap();
        if let Some(env) = env.get(&key) {
          Some(env)
        } else {
          None
        }
    }
}

impl PluginModule for ExtPluginModule {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        invoker: Arc<dyn polywrap_client::core::invoke::Invoker>,
    ) -> Result<Vec<u8>, polywrap_plugin::error::PluginError> {
        let method_name = method_name.to_string();
        let method_name = CString::new(method_name).unwrap();
        let method_name_ptr = method_name.into_raw();

        let params_raw_parts = params.to_vec().into_raw_parts();
        let params_buffer = params_raw_parts[0];
        let params_len = params_raw_parts[1];

        (self._wrap_invoke)(method_name_ptr, params_buffer, params_len, invoker)
    }
}

pub fn create_plugin_wrapper(
  wasm_module_buffer: *const u8,
  wasm_module_len: usize,
  file_reader_ptr: *mut SimpleFileReader
) {
  let file_reader = Arc::from(instantiate_from_ptr_and_take_ownership(file_reader_ptr) as Box<dyn FileReader>);
  let wasm_module = unsafe {
      from_raw_parts(wasm_module_buffer, wams_module_len)
  };
  let wrapper = WasmWrapper::new(wasm_module.to_vec(), file_reader);
}