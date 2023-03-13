use std::sync::{Arc, Mutex};

use polywrap_client::core::{resolvers::{uri_resolver_like::UriResolverLike, uri_resolution_context::{UriPackage, UriWrapper}}, client::UriRedirect, uri::Uri};
use polywrap_plugin::{package::PluginPackage, wrapper::PluginWrapper};
use polywrap_wasm::{wasm_package::WasmPackage, wasm_wrapper::WasmWrapper};

use crate::utils::{get_string_from_cstr_ptr, instantiate_from_ptr};

use super::uri_resolver::UriResolversVariant;

#[repr(C)]
struct Redirect {
  from: *const std::ffi::c_char,
  to: *const std::ffi::c_char
}

#[repr(C)]
pub enum UriResolverLikeType {
  Resolver,
  Redirect,
  WasmPackage,
  PluginPackage,
  WasmWrapper,
  PluginWrapper,
}

#[repr(C)]
pub struct UriResolverLikeVariant {
  _type: UriResolverLikeType,
  data: *mut std::ffi::c_void,
  uri: *const std::ffi::c_char
}

impl From<UriResolverLikeVariant> for UriResolverLike {
    fn from(value: UriResolverLikeVariant) -> Self {
      let data = instantiate_from_ptr(value.data as *mut UriResolverLikeVariant);
      let uri: Uri = get_string_from_cstr_ptr(data.uri).try_into().unwrap();
  
      match data._type {
        UriResolverLikeType::Resolver => {
          let uri_resolver_variant = instantiate_from_ptr(value.data as *mut UriResolversVariant);
          UriResolverLike::Resolver(uri_resolver_variant.into())
        },
        UriResolverLikeType::Redirect => {
          let redirect = instantiate_from_ptr(value.data as *mut Redirect);
          let from = get_string_from_cstr_ptr(redirect.from).try_into().unwrap();
          let to = get_string_from_cstr_ptr(redirect.to).try_into().unwrap();
          UriResolverLike::Redirect(UriRedirect::new(from, to))
        },
        UriResolverLikeType::WasmPackage => {
          let package = instantiate_from_ptr(value.data as *mut WasmPackage);
          UriResolverLike::Package(UriPackage {
            uri,
            package: Arc::new(Mutex::new(package))
          })
        },
        UriResolverLikeType::PluginPackage => {
          let package = instantiate_from_ptr(value.data as *mut PluginPackage);
          UriResolverLike::Package(UriPackage {
            uri,
            package: Arc::new(Mutex::new(package))
          })
        },
        UriResolverLikeType::WasmWrapper => {
          let wrapper = instantiate_from_ptr(value.data as *mut WasmWrapper);
          UriResolverLike::Wrapper(UriWrapper {
            uri,
            wrapper: Arc::new(Mutex::new(wrapper))
          })
        },
        UriResolverLikeType::PluginWrapper => {
          let wrapper = instantiate_from_ptr(value.data as *mut PluginWrapper);
          UriResolverLike::Wrapper(UriWrapper {
            uri,
            wrapper: Arc::new(Mutex::new(wrapper))
          })
        }
      }
    }
}
