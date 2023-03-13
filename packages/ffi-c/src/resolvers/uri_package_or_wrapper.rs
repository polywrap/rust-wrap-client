use std::sync::{Arc, Mutex};

use polywrap_client::core::{resolvers::uri_resolution_context::UriPackageOrWrapper, uri::Uri};
use polywrap_plugin::{wrapper::PluginWrapper, package::PluginPackage};
use polywrap_wasm::{wasm_wrapper::WasmWrapper, wasm_package::WasmPackage};

use crate::utils::{get_string_from_cstr_ptr, instantiate_from_ptr};

#[repr(C)]
pub enum SafeUriPackageOrWrapperType {
  Uri,
  WasmWrapper,
  PluginWrapper,
  WasmPackage,
  PluginPackage
}

#[repr(C)]
pub struct SafeUriPackageOrWrapper {
  uri: *const std::ffi::c_char,
  data_type: SafeUriPackageOrWrapperType,
  data: *mut std::ffi::c_void
}

impl From<SafeUriPackageOrWrapper> for UriPackageOrWrapper {
    fn from(value: SafeUriPackageOrWrapper) -> Self {
      let entry = instantiate_from_ptr(value.data as *mut SafeUriPackageOrWrapper);
      let entry_uri: Uri = get_string_from_cstr_ptr(entry.uri).try_into().unwrap();
      match entry.data_type {
        SafeUriPackageOrWrapperType::Uri => UriPackageOrWrapper::Uri(entry_uri.clone()),
        SafeUriPackageOrWrapperType::WasmWrapper => {
          let wrapper = instantiate_from_ptr(entry.data as *mut WasmWrapper);
          UriPackageOrWrapper::Wrapper(entry_uri.clone(), Arc::new(Mutex::new(wrapper)))
        },
        SafeUriPackageOrWrapperType::PluginWrapper => {
          let wrapper = instantiate_from_ptr(entry.data as *mut PluginWrapper);
          UriPackageOrWrapper::Wrapper(entry_uri.clone(), Arc::new(Mutex::new(wrapper)))
        },
        SafeUriPackageOrWrapperType::WasmPackage => {
          let package = instantiate_from_ptr(entry.data as *mut WasmPackage);
          UriPackageOrWrapper::Package(entry_uri.clone(), Arc::new(Mutex::new(package)))
        },
        SafeUriPackageOrWrapperType::PluginPackage => {
          let package = instantiate_from_ptr(entry.data as *mut PluginPackage);
          UriPackageOrWrapper::Package(entry_uri.clone(), Arc::new(Mutex::new(package)))
        }
      }
    }
}