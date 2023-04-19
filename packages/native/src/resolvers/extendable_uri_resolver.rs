use polywrap_client::resolvers::extendable_uri_resolver::ExtendableUriResolver;

use crate::utils::into_raw_ptr_and_forget;

#[no_mangle]
pub extern "C" fn create_extendable_resolver() -> *mut ExtendableUriResolver {
  into_raw_ptr_and_forget(ExtendableUriResolver::new(None)) as *mut ExtendableUriResolver
}

