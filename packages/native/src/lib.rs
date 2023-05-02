pub mod builder;
pub mod resolvers;
pub mod invoker;
pub mod wasm_wrapper;
pub mod plugin_wrapper;
pub mod loader;
pub mod client;
pub mod wrapper;
pub mod package;

use std::sync::Arc;

use crate::resolvers::ffi_resolver::FFIUriResolver;
use crate::resolvers::uri_package_or_wrapper::FFIUriPackageOrWrapper;
use crate::client::FFILoader;
use crate::plugin_wrapper::FFIPluginModule;
use crate::invoker::FFIInvoker;
use crate::client::FFIClient;
use crate::resolvers::recursive::FFIRecursiveUriResolver;
use crate::resolvers::_static::FFIStaticUriResolver;
use crate::resolvers::extendable::FFIExtendableUriResolver;
use crate::plugin_wrapper::FFIPluginWrapper;
use crate::wasm_wrapper::FFIWasmWrapper;
use crate::resolvers::uri_package_or_wrapper::FFIUriPackageOrWrapperKind;
use crate::wrapper::FFIWrapper;
use crate::package::FFIWrapPackage;
use crate::resolvers::uri_package_or_wrapper::FFIUriPackageOrWrapperUriVariant;
use crate::resolvers::uri_package_or_wrapper::FFIUriPackageOrWrapperWrapperVariant;
use crate::resolvers::uri_package_or_wrapper::FFIUriPackageOrWrapperPackageVariant;
use crate::resolvers::uri_resolver_like::FFIUriResolverLikeRedirectVariant;
use crate::resolvers::uri_resolver_like::FFIUriResolverLikeResolverVariant;
use crate::resolvers::uri_resolver_like::FFIUriResolverLikeWrapperVariant;
use crate::resolvers::uri_resolver_like::FFIUriResolverLikeKind;
use crate::resolvers::uri_resolver_like::FFIUriResolverLikePackageVariant;
use crate::resolvers::uri_resolver_like::FFIUriResolverLikeResolverLikeVariant;
use crate::resolvers::uri_resolver_like::FFIUriResolverLike;
use polywrap_client::core::uri::Uri;
use polywrap_client::core::error::Error;
use crate::builder::FFIBuilderConfig;

pub fn uri_from_string(uri: &str) -> Result<Arc<Uri>, Error> {
  Ok(Arc::new(uri.to_string().try_into()?))
}

uniffi::include_scaffolding!("main");