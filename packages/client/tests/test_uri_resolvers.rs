#![feature(trait_upcasting)]
use std::any::TypeId;

use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_client_builder::types::{BuilderConfig, ClientConfigHandler};
use polywrap_resolvers::{uri_resolver_wrapper::UriResolverWrapper};

use polywrap_core::{uri::Uri, resolvers::{uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}, resolver_with_history::ResolverWithHistory}};
use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_wasm::wasm_wrapper::WasmWrapper;

#[tokio::test]
async fn test_uri_resolver_wrapper() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let wrapper_path = format!("{}/subinvoke/00-subinvoke/implementations/as", path);
    let wrapper_uri = Uri::try_from(format!("fs/{}", wrapper_path)).unwrap();

    let builder = BuilderConfig::new(None);
    let config = builder.build();
    let client = PolywrapClient::new(config);
    
    let mut uri_resolution_context = UriResolutionContext::new();
    let uri_resolver_wrapper = UriResolverWrapper::new(
        Uri::try_from("wrap://ens/fs-resolver.polywrap.eth").unwrap()
    );
    let result = uri_resolver_wrapper._try_resolve_uri(
        &wrapper_uri.clone(), 
        &client.loader, 
        &mut uri_resolution_context
    ).await;

    if result.is_err() {
        panic!("Error in try resolver uri: {:?}", result.err());
    }

    let result = result.unwrap();
    if let UriPackageOrWrapper::Wrapper(_, w) = result {
        let wrapper = w.lock().await;
        let wrapper = &*wrapper as &dyn std::any::Any;
        assert_eq!(wrapper.type_id(), TypeId::of::<WasmWrapper>());
    } else {
        panic!("Expected wrapper, got package or uri");
    }
}

#[tokio::test]
async fn test_recursive_uri_resolver() {
    let wrapper_github_path = "https://raw.githubusercontent.com/polywrap/wrap-test-harness/v0.2.1/wrappers/subinvoke/00-subinvoke/implementations/as";
    let http_wrapper_uri = Uri::try_from(format!("http/{}", wrapper_github_path)).unwrap();

    let builder = BuilderConfig::new(None);
    let config = builder.build();
    let client = PolywrapClient::new(config);

    let mut uri_resolution_context = UriResolutionContext::new();
    let result = client.loader.resolver.try_resolve_uri(
        &http_wrapper_uri.clone(), 
        &client.loader, 
        &mut uri_resolution_context
    ).await;

    if result.is_err() {
        panic!("Error in try resolver uri: {:?}", result.err());
    }

    let result = result.unwrap();
    if let UriPackageOrWrapper::Wrapper(_, w) = result {
        let wrapper = w.lock().await;
        let wrapper = &*wrapper as &dyn std::any::Any;
        assert_eq!(wrapper.type_id(), TypeId::of::<WasmWrapper>());
    } else {
        panic!("Expected wrapper, got package or uri");
    }
}