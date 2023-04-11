use std::{collections::HashMap};

use polywrap_client::core::{resolvers::{static_resolver::StaticResolver, uri_resolution_context::UriPackageOrWrapper}};
use crate::utils::{instantiate_from_ptr, into_raw_ptr_and_forget};
use super::uri_package_or_wrapper::SafeUriPackageOrWrapper;

#[no_mangle]
pub extern "C" fn create_static_resolver(entries: *const SafeUriPackageOrWrapper, len: usize) -> *mut StaticResolver {
  let mut uri_map: HashMap<String, UriPackageOrWrapper> = HashMap::new();
  for i in 0..len {
    let entry_ptr = unsafe { entries.add(i) };
    let entry = instantiate_from_ptr(entry_ptr as *mut SafeUriPackageOrWrapper);
    let uri_package_or_wrapper: UriPackageOrWrapper = entry.into();

    uri_map.insert(uri_package_or_wrapper.uri().to_string(), uri_package_or_wrapper);
  }

  let static_resolver = StaticResolver::new(uri_map);

  into_raw_ptr_and_forget(static_resolver) as *mut StaticResolver
}
