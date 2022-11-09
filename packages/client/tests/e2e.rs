use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::{
    client::{ClientConfig, UriRedirect},
    invoke::InvokeArgs, uri::Uri,
};
use polywrap_msgpack::msgpack;
use polywrap_resolvers::{
    base::BaseResolver, filesystem::FilesystemResolver, redirects::RedirectsResolver, static_resolver::StaticResolver,
};
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_tests::helpers::get_tests_path;
use std::{sync::Arc};

#[tokio::test]
async fn subinvoke_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let subinvoke_uri: Uri = format!("fs/{}/subinvoke/00-subinvoke/implementations/as", path).try_into().unwrap();
    let invoke_uri: Uri = format!("fs/{}/subinvoke/01-invoke/implementations/as", path).try_into().unwrap();

    let redirects = vec![UriRedirect::new(
        "ens/add.eth".try_into().unwrap(),
        subinvoke_uri.clone(),
    )];
    let file_reader = SimpleFileReader::new();
    let client = PolywrapClient::new(ClientConfig {
        redirects: vec![],
        resolver: Arc::new(BaseResolver::new(
            Box::new(FilesystemResolver::new(Arc::new(file_reader))),
            Box::new(RedirectsResolver::new(redirects)),
        )),
    });

    let invoke_args = InvokeArgs::Msgpack(msgpack!({"a": 1, "b": 1}));

    let invoke_opts = polywrap_core::invoke::InvokeOptions {
        args: Some(&invoke_args),
        env: None,
        resolution_context: None,
        uri: &invoke_uri,
        method: "addAndIncrement",
    };


    assert_eq!(invoke_result, 3);
}


async fn invoke_with_static_resolver() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let subinvoke_uri: Uri = format!("fs/{}/subinvoke/00-subinvoke/implementations/as", path).try_into().unwrap();
    let invoke_uri: Uri = format!("fs/{}/subinvoke/01-invoke/implementations/as", path).try_into().unwrap();

    // let redirects = vec![UriRedirect::new(
    //     "ens/add.eth".try_into().unwrap(),
    //     subinvoke_uri.clone(),
    // )];

    let module_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.wasm", path);
    let module = WasmModule::Path(module_path);
    let manifest_path = format!("{}/subinvoke/00-subinvoke/implementations/as/wrap.info", path);
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();
    let manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();
    let file_reader = SimpleFileReader::new();
    let wrapper = WasmWrapper::new(module, Arc::new(file_reader), manifest);
    
    let uri_wrapper = UriWrapper {
        uri: Uri::new("ens/wrapper.eth"),
        wrapper: w
      };
    
    // let file_reader = SimpleFileReader::new();
    let static_resolver = StaticResolver::_from(vec![uri_wrapper]);

    let client = PolywrapClient::new(ClientConfig {
        redirects: vec![],
        resolver: Arc::new(static_resolver),
    });


    let invoke_args = InvokeArgs::Msgpack(msgpack!({"a": 1, "b": 1}));

    let invoke_opts = polywrap_core::invoke::InvokeOptions {
        args: Some(&invoke_args),
        env: None,
        resolution_context: None,
        uri: &Uri::new("ens/wrapper.eth"),
        method: "addAndIncrement",
    };


    assert_eq!(invoke_result, 3);
}