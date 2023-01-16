use std::{fs, path::Path, sync::Arc};

use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_client_builder::types::{BuilderConfig, ClientConfigHandler};
use polywrap_resolvers::{uri_resolver_wrapper::UriResolverWrapper};

use polywrap_core::{uri::Uri, resolvers::{uri_resolution_context::{UriResolutionContext, UriPackageOrWrapper}, resolver_with_history::ResolverWithHistory}, file_reader::SimpleFileReader};
use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_wasm::wasm_wrapper::WasmWrapper;
use wrap_manifest_schemas::deserialize::deserialize_wrap_manifest;

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

    if let Ok(UriPackageOrWrapper::Wrapper(_, _w)) = result {
        
    }
    
}

#[tokio::test]
async fn recursive_uri_resolver() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let wrapper_local_path = format!("{}/subinvoke/00-subinvoke/implementations/as", path);

    let module_path = format!("{}/wrap.wasm", wrapper_local_path);
    let manifest_path = format!("{}/wrap.info", wrapper_local_path);

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();

    let manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();
    let file_reader = SimpleFileReader::new();

    let mock_wrapper = WasmWrapper::new(
        module_bytes,
        Arc::new(file_reader),
        manifest
    );

    let wrapper_github_path = "https://raw.githubusercontent.com/polywrap/wasm-test-harness/v0.2.1/wrappers/subinvoke/00-subinvoke/implementations/as";
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

    if let Ok(r) = result {
        if let UriPackageOrWrapper::Wrapper(_, _w) = r {
            let gg = _w.lock().await;
            let z = _w.lock().await;
            assert_eq!(_w.lock().await, mock_wrapper);
        }
    } else {
      println!("{:?}", result.err().unwrap());
    }
    // assert_eq!(true, false);
}