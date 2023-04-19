use std::{sync::{Arc, Mutex}};

use polywrap_client::core::{file_reader::{SimpleFileReader, FileReader}};
use polywrap_plugin::wrapper::PluginWrapper;
use polywrap_wasm::wasm_wrapper::WasmWrapper;

use crate::{utils::{into_raw_ptr_and_forget, instantiate_from_ptr_and_take_ownership, instantiate_from_ptr, Buffer}, ext_plugin::ExtPluginModule};

pub fn create_simple_file_reader() -> *const SimpleFileReader {
  let reader = SimpleFileReader::new();
  into_raw_ptr_and_forget(reader) as *mut SimpleFileReader
}

pub fn create_wasm_wrapper(
  wasm_module_buffer: *const Buffer,
  file_reader_ptr: *mut SimpleFileReader
) -> *mut WasmWrapper {
  let file_reader = Arc::from(instantiate_from_ptr_and_take_ownership(file_reader_ptr) as Box<dyn FileReader>);
  let wasm_module: Vec<u8> = instantiate_from_ptr(wasm_module_buffer as *mut Buffer).into();
  let wasm_wrapper = WasmWrapper::new(wasm_module, file_reader);

  into_raw_ptr_and_forget(wasm_wrapper) as *mut WasmWrapper
}

pub fn create_plugin_wrapper(
  ext_plugin_ptr: *mut ExtPluginModule
) -> *mut PluginWrapper {
  let plugin_module = Box::new(instantiate_from_ptr(ext_plugin_ptr));
  let plugin_wrapper = PluginWrapper::new(
    Arc::new(
      Mutex::new(plugin_module)
    )
  );

  into_raw_ptr_and_forget(plugin_wrapper) as *mut PluginWrapper
}