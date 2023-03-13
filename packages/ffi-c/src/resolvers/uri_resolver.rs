use polywrap_client::core::resolvers::{uri_resolver::UriResolver, recursive_resolver::RecursiveResolver, static_resolver::StaticResolver};

use crate::utils::instantiate_from_ptr;

#[repr(C)]
pub enum UriResolversType {
  Static,
  Recursive,
}

#[repr(C)]
pub struct UriResolversVariant {
  _type: UriResolversType,
  data: *const std::ffi::c_void
}

impl From<UriResolversVariant> for Box<dyn UriResolver> {
    fn from(value: UriResolversVariant) -> Self {
        match value._type {
          UriResolversType::Recursive => {
            Box::new(instantiate_from_ptr(value.data as *mut RecursiveResolver))
          }
          UriResolversType::Static => {
            Box::new(instantiate_from_ptr(value.data as *mut StaticResolver))
          },
        }
    }
}