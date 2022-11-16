use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::{
    client::{ClientConfig, UriRedirect},
    invoke::InvokeArgs,
    uri::Uri,
};
use polywrap_msgpack::msgpack;
use polywrap_resolvers::{
    base::BaseResolver, filesystem::FilesystemResolver, redirects::RedirectsResolver,
};
use polywrap_tests::helpers::get_tests_path;
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::test]
async fn subinvoke_test() {
    let subinvoke_uri: Uri = Uri::try_from("fs/tests/cases/simple-subinvoke/subinvoke").unwrap()
        .try_into()
        .unwrap();
    let invoke_uri: Uri = Uri::try_from("fs/tests/cases/simple-subinvoke/invoke").unwrap()
        .try_into()
        .unwrap();

    let redirects = vec![UriRedirect::new(
        "ens/add.eth".try_into().unwrap(),
        subinvoke_uri.clone(),
    )];
    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));
    let redirects_resolver = RedirectsResolver::new(redirects);
    let client = PolywrapClient::new(ClientConfig {
        redirects: vec![],
        resolver: Arc::new(Mutex::new(BaseResolver::new(
            Box::new(fs_resolver),
            Box::new(redirects_resolver),
        ))),
    });

    let invoke_args = InvokeArgs::Msgpack(msgpack!({"a": 1, "b": 1}));

    let invoke_result = client
        .invoke_and_decode::<String>(&invoke_uri, "add", Some(&invoke_args), None)
        .await
        .unwrap();

    assert_eq!(invoke_result, "1 + 1 = 2");
}
