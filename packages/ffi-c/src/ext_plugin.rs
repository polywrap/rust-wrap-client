use std::{ffi::{c_char, c_void}, sync::Arc, ptr::null};

use polywrap_client::{
    core::{env::Env, invoke::Invoker}, client::PolywrapClient,
};
use polywrap_plugin::module::{PluginModule, PluginWithEnv};
use serde_json::json;

use crate::utils::{
    get_string_from_cstr_ptr, instantiate_from_ptr, into_raw_ptr_and_forget, raw_ptr_from_str, Buffer,
};

// TODO: HACK. Find a better way to dynamically retrieve methods

struct MethodsMap(*const c_void);

unsafe impl Send for MethodsMap {}
unsafe impl Sync for MethodsMap {}

type ExtPluginWrapInvokeFn = extern "C" fn(
  method_name: *const i8,
  params_buffer: *const u8,
  params_len: usize,
  invoker: *mut PolywrapClient,
  methods_map: *const c_void
) -> Buffer;

#[repr(C)]
pub struct ExtPluginModule {
    env: Env,
    methods_map: MethodsMap,
    _wrap_invoke: Option<ExtPluginWrapInvokeFn>,
}

impl PluginWithEnv for ExtPluginModule {
    fn set_env(&mut self, env: Env) {
        self.env = env;
    }

    fn get_env(&self, key: String) -> Option<&Env> {
        if let Some(env) = self.env.get(&key) {
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
        invoker: Arc<(dyn Invoker + 'static)>,
    ) -> Result<Vec<u8>, polywrap_plugin::error::PluginError> {
        let method_name = method_name.to_string();
        let method_name_ptr = raw_ptr_from_str(&method_name);

        let mut params_vec = params.to_vec();
        let invoker_ptr = into_raw_ptr_and_forget(invoker);
        let methods = self.methods_map.0;

        if let Some(invoke_fn_ptr) = self._wrap_invoke {
          let result = (invoke_fn_ptr)(
            method_name_ptr,
            params_vec.as_mut_ptr(),
            params_vec.len(),
            invoker_ptr as _,
            methods
          );

        Ok(result.into())
      } else {
        Err(polywrap_plugin::error::PluginError::ModuleError("_wrap_invoke fn pointer for ExtPlugin not set".to_owned()))
      }
    }
}

#[no_mangle]
pub extern "C" fn set_plugin_env(plugin_ptr: *mut ExtPluginModule, env_json_str: *const c_char) {
    let env_json_str = get_string_from_cstr_ptr(env_json_str);
    let new_plugin_env: Env = serde_json::from_str(&env_json_str).unwrap();

    let mut plugin = instantiate_from_ptr(plugin_ptr);
    plugin.set_env(new_plugin_env);
}

//TODO: handle optional types
#[no_mangle]
pub extern "C" fn get_plugin_env(
    plugin_ptr: *mut ExtPluginModule,
    key: *const c_char,
) -> *const i8 {
    let key_str = get_string_from_cstr_ptr(key);
    let plugin = instantiate_from_ptr(plugin_ptr);

    if let Some(value) = plugin.get_env(key_str) {
        let value_string = value.to_string();
        raw_ptr_from_str(&value_string)
    } else {
        null()
    }
}

#[no_mangle]
pub extern "C" fn set_plugin_wrap_invoke(plugin_ptr: *mut ExtPluginModule, fn_ptr: ExtPluginWrapInvokeFn) {
    let mut plugin = instantiate_from_ptr(plugin_ptr);
    plugin._wrap_invoke = Some(fn_ptr);
}

#[no_mangle]
pub extern "C" fn create_plugin(method_map_ptr: *const c_void) -> *mut ExtPluginModule {
  let plugin = ExtPluginModule {
    env: json!({}),
    methods_map: MethodsMap(method_map_ptr),
    _wrap_invoke: None
  };

  into_raw_ptr_and_forget(plugin) as *mut _
}