use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::{
    client::{ClientConfig, UriRedirect},
    invoke::InvokeArgs, uri::Uri,
};
use polywrap_msgpack::msgpack;
use polywrap_resolvers::{
    base::BaseResolver, filesystem::FilesystemResolver, redirects::RedirectsResolver,
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
        envs: None
    });

    let invoke_args = InvokeArgs::Msgpack(msgpack!({"a": 1, "b": 1}));

    let invoke_opts = polywrap_core::invoke::InvokeOptions {
        args: Some(&invoke_args),
        env: None,
        resolution_context: None,
        uri: &invoke_uri,
        method: "addAndIncrement",
    };

    let invoke_result = client.invoke_and_decode::<i32>(&invoke_opts).await.unwrap();

    assert_eq!(invoke_result, 3);
}
