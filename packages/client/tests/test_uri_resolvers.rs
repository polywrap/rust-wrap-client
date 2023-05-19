#![feature(trait_upcasting)]
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;

use polywrap_client::client::PolywrapClient;
use polywrap_client::resolvers::uri_resolver_wrapper::UriResolverWrapper;

use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::interface_implementation::InterfaceImplementations;
use polywrap_core::resolution::uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper};
use polywrap_core::resolution::uri_resolver::UriResolver;
use polywrap_core::uri::Uri;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_resolvers::static_resolver::StaticResolver;
use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_wasm::wasm_wrapper::WasmWrapper;

// #[test]
// fn test_uri_resolver_wrapper() {
//     let test_path = get_tests_path().unwrap();
//     let path = test_path.into_os_string().into_string().unwrap();
//     let wrapper_path = format!("{path}/subinvoke/00-subinvoke/implementations/as");
//     let wrapper_uri = Uri::try_from(format!("fs/{wrapper_path}")).unwrap();

//     let file_reader = SimpleFileReader::new();
//     let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

//     let base_resolver = BaseResolver::new(
//         Box::new(fs_resolver),
//         Box::new(StaticResolver::new(HashMap::new())),
//     );
//     let mut interfaces: InterfaceImplementations = HashMap::new();
//     interfaces.insert(
//         "wrap://ens/uri-resolver.core.polywrap.eth".to_string(),
//         vec![Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap()],
//     );
//     let config = ClientConfig {
//         envs: None,
//         resolver: Arc::new(base_resolver),
//         interfaces: Some(interfaces),
//     };
//     let mut uri_resolution_context = UriResolutionContext::new();
//     let uri_resolver_wrapper =
//         UriResolverWrapper::new(Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap());
//     let client = PolywrapClient::new(config);
//     let result =
//         uri_resolver_wrapper.try_resolve_uri(&wrapper_uri, Arc::new(client), &mut uri_resolution_context);

//     if result.is_err() {
//         panic!("Error in try resolver uri: {:?}", result.err());
//     }

//     let result = result.unwrap();
//     if let UriPackageOrWrapper::Wrapper(_, wrapper) = result {
//         let wrapper = &*wrapper as &dyn std::any::Any;
//         assert_eq!(wrapper.type_id(), TypeId::of::<WasmWrapper>());
//     } else {
//         panic!("Expected wrapper, got package or uri");
//     }
// }

// #[test]
// fn test_recursive_uri_resolver() {
//     let wrapper_github_path = "https://raw.githubusercontent.com/polywrap/wrap-test-harness/v0.2.1/wrappers/subinvoke/00-subinvoke/implementations/as";
//     let http_wrapper_uri = Uri::try_from(format!("http/{}", wrapper_github_path)).unwrap();

//     let builder = BuilderConfig::new(None);
//     let config = builder.build();
//     let client = PolywrapClient::new(config);

//     let mut uri_resolution_context = UriResolutionContext::new();
//     let result = client.resolver.try_resolve_uri(
//         &http_wrapper_uri,
//         &client,
//         &mut uri_resolution_context
//     );

//     if result.is_err() {
//         panic!("Error in try resolver uri: {:?}", result.err());
//     }

//     let result = result.unwrap();
//     if let UriPackageOrWrapper::Wrapper(_, w) = result {
//         let wrapper = w.lock().unwrap();
//         let wrapper = &*wrapper as &dyn std::any::Any;
//         assert_eq!(wrapper.type_id(), TypeId::of::<WasmWrapper>());
//     } else {
//         panic!("Expected wrapper, got package or uri");
//     }
// }

// #[test]
// fn test_ipfs_uri_resolver_extension() {
//     let wrapper_uri = Uri::try_from("wrap://ipfs/QmaM318ABUXDhc5eZGGbmDxkb2ZgnbLxigm5TyZcCsh1Kw").unwrap();

//     let mut builder = BuilderConfig::new(None);
//     builder.add_env(wrapper_uri.clone(), json!({
//         "provider": "https://ipfs.wrappers.io",
//         "fallbackProviders": ["https://ipfs.io"],
//         "retries": { "tryResolveUri": 2, "getFile": 2 },
//       }));
//     let config = builder.build();
//     let client = PolywrapClient::new(config);

//     let result = client.try_resolve_uri(&wrapper_uri, None);

//     if result.is_err() {
//         panic!("Error in try_resolve_uri: {:?}", result.err());
//     }

//     let result = result.unwrap();
//     if let UriPackageOrWrapper::Wrapper(_, w) = result {
//         let wrapper = w.lock().unwrap();
//         let wrapper = &*wrapper as &dyn std::any::Any;
//         assert_eq!(wrapper.type_id(), TypeId::of::<WasmWrapper>());
//     } else {
//         panic!("Expected wrapper, got package or uri");
//     }
// }
