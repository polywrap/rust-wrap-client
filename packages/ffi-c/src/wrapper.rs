use std::{slice::from_raw_parts, sync::Arc};

use polywrap_client::core::{file_reader::{SimpleFileReader, FileReader}};
use polywrap_wasm::wasm_wrapper::WasmWrapper;

use crate::utils::{into_raw_ptr_and_forget, instantiate_from_ptr_and_take_ownership};

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
