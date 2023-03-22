use std::{ffi::{c_char}, sync::Arc};

use polywrap_client::{
    core::{env::Env, invoke::Invoker}, client::PolywrapClient,
};
use polywrap_plugin::module::{PluginModule, PluginWithEnv};

use crate::utils::{
    get_string_from_cstr_ptr, instantiate_from_ptr, into_raw_ptr_and_forget, raw_ptr_from_str, SafeOption, Buffer,
};

#[repr(C)]
pub struct ExtPluginModule {
    env: Env,

    _wrap_invoke: extern "C" fn(
        method_name: *const i8,
        params_buffer: *const u8,
        params_len: usize,
        invoker: *mut PolywrapClient,
    ) -> Buffer,
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

        let result = (self._wrap_invoke)(
            method_name_ptr,
            params_vec.as_mut_ptr(),
            params_vec.len(),
            invoker_ptr as _,
        );

        Ok(result.into())
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
) -> SafeOption<*const i8> {
    let key_str = get_string_from_cstr_ptr(key);
    let plugin = instantiate_from_ptr(plugin_ptr);

    if let Some(value) = plugin.get_env(key_str) {
        let value_string = value.to_string();
        SafeOption::Some(raw_ptr_from_str(&value_string))
    } else {
        SafeOption::None
    }
}
