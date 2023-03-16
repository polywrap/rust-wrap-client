use polywrap_client::core::{resolvers::{uri_resolver_like::UriResolverLike, recursive_resolver::RecursiveResolver}};
use crate::utils::{instantiate_from_ptr, into_raw_ptr_and_forget};
use super::uri_resolver_like::SafeUriResolverLikeVariant;

pub extern "C" fn create_recursive_resolver(entries: *const SafeUriResolverLikeVariant, len: usize) -> *mut RecursiveResolver {
  let mut uri_resolvers: Vec<UriResolverLike> = Vec::new();
  
  for i in 0..len {
    let entry_ptr = unsafe { entries.add(i) };
    let entry = instantiate_from_ptr(entry_ptr as *mut SafeUriResolverLikeVariant);

    uri_resolvers.push(entry.into());
  };

  let recursive_resolver = RecursiveResolver::from(uri_resolvers);
  into_raw_ptr_and_forget(recursive_resolver) as *mut RecursiveResolver
}