pub mod builder;
pub mod resolvers;
pub mod invoker;
pub mod wasm_wrapper;
pub mod client;
pub mod wrapper;
pub mod package;
pub mod uri;

use polywrap_client::core::error::Error;

use resolvers::ffi_resolver::FFIUriResolver;
use resolvers::uri_package_or_wrapper::FFIUriPackageOrWrapper;
use invoker::FFIInvoker;
use client::FFIClient;
use resolvers::recursive::FFIRecursiveUriResolver;
use resolvers::_static::FFIStaticUriResolver;
use resolvers::extendable::FFIExtendableUriResolver;
use wasm_wrapper::FFIWasmWrapper;
use resolvers::uri_package_or_wrapper::FFIUriPackageOrWrapperKind;
use wrapper::FFIWrapper;
use package::FFIWrapPackage;
use resolvers::uri_package_or_wrapper::FFIUriPackageOrWrapperUriVariant;
use resolvers::uri_package_or_wrapper::FFIUriPackageOrWrapperWrapperVariant;
use resolvers::uri_package_or_wrapper::FFIUriPackageOrWrapperPackageVariant;
use resolvers::uri_resolver_like::FFIUriResolverLikeRedirectVariant;
use resolvers::uri_resolver_like::FFIUriResolverLikeResolverVariant;
use resolvers::uri_resolver_like::FFIUriResolverLikeWrapperVariant;
use resolvers::uri_resolver_like::FFIUriResolverLikeKind;
use resolvers::uri_resolver_like::FFIUriResolverLikePackageVariant;
use resolvers::uri_resolver_like::FFIUriResolverLikeResolverLikeVariant;
use resolvers::uri_resolver_like::FFIUriResolverLike;
use uri::FFIUri;
use builder::FFIBuilderConfig;

uniffi::include_scaffolding!("main");