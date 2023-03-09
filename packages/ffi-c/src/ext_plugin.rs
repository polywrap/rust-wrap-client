use std::ffi::CString;

use polywrap_client::wrapper_invoker::WrapperInvoker;
use polywrap_plugin::module::{PluginWithEnv, PluginModule};

use crate::utils::get_string_from_cstr_ptr;

#[repr(C)]
struct ExtPluginModule {
  env: *mut std::ffi::c_char,

  _wrap_invoke: extern "C" fn(
    method_name: *const std::ffi::c_char,
    params_buffer: *const u8,
    params_len: usize,
    invoker: *mut WrapperInvoker,
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
        invoker: *mut WrapperInvoker,
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
