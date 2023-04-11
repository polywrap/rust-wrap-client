use polywrap_client::core::resolvers::{uri_resolver::UriResolver, recursive_resolver::RecursiveResolver, static_resolver::StaticResolver};

use crate::utils::instantiate_from_ptr;

#[repr(C)]
pub enum SafeUriResolversType {
  Static,
  Recursive,
}

#[repr(C)]
pub struct SafeUriResolversVariant {
  _type: SafeUriResolversType,
  data: *const std::ffi::c_void
}

impl From<SafeUriResolversVariant> for Box<dyn UriResolver> {
    fn from(value: SafeUriResolversVariant) -> Self {
        match value._type {
          SafeUriResolversType::Recursive => {
            Box::new(instantiate_from_ptr(value.data as *mut RecursiveResolver))
          }
          SafeUriResolversType::Static => {
            Box::new(instantiate_from_ptr(value.data as *mut StaticResolver))
          },
        }
    }
}