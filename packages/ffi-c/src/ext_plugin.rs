use std::{ffi::{c_void}, sync::Arc, fmt::{Debug, Formatter}};

use polywrap_client::{
    core::{env::Env, invoke::Invoker}, client::PolywrapClient, msgpack::extensions::generic_map::convert_msgpack_to_json,
};
use polywrap_plugin::module::{PluginModule};
use serde_json::{json, Value};

use crate::utils::{
    get_string_from_cstr_ptr, into_raw_ptr_and_forget, raw_ptr_from_str,
};

struct PluginPtrHandle(*const c_void);

unsafe impl Send for PluginPtrHandle {}
unsafe impl Sync for PluginPtrHandle {}

pub type PluginInvokeFn = extern "C" fn(
  plugin_ptr: *const c_void,
  method_name: *const i8,
  params: *const i8,
  env: Option<Env>,
  invoker: *mut PolywrapClient
) -> *const i8;

#[derive(Debug)]
#[repr(C)]
pub struct ExtPluginModule {
    env: Env,
    ptr_handle: PluginPtrHandle,
    plugin_invoke: PluginInvokeFn
}

impl ExtPluginModule {
    pub fn new(plugin_ptr: *const c_void, plugin_invoke: PluginInvokeFn) -> Self {
      ExtPluginModule {
        env: json!({}),
        ptr_handle: PluginPtrHandle(plugin_ptr),
        plugin_invoke,
      }
    }
}

impl PluginModule for ExtPluginModule {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        env: Option<Env>,
        invoker: Arc<(dyn Invoker + 'static)>,
    ) -> Result<Vec<u8>, polywrap_plugin::error::PluginError> {
        let method_name = method_name.to_string();
        let method_name_ptr = raw_ptr_from_str(&method_name);

        let params_vec = params.to_vec();
        let params_msgpack: polywrap_client::msgpack::Value = polywrap_client::msgpack::decode(&params_vec).unwrap();
        let params_json = convert_msgpack_to_json(params_msgpack);
        let params_json_string = params_json.to_string();
        let params_ptr = raw_ptr_from_str(&params_json_string);
        let invoker_ptr = into_raw_ptr_and_forget(invoker);

        let result_cstr = (self.plugin_invoke)(
          self.ptr_handle.0,
          method_name_ptr,
          params_ptr,
          env,
          invoker_ptr as _,
        );

        let result_string = get_string_from_cstr_ptr(result_cstr);
        let result: Value = serde_json::from_str(&result_string).unwrap();
        let result_buffer = polywrap_client::msgpack::serialize(result).unwrap();

        Ok(result_buffer)
    }
}
