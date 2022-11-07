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
use std::{sync::Arc};

#[tokio::test]
async fn subinvoke_test() {
    let redirects = vec![UriRedirect::new(
        "ens/add.eth".try_into().unwrap(),
        "fs/tests/cases/simple-subinvoke/subinvoke".try_into().unwrap(),
    )];
    let file_reader = SimpleFileReader::new();
    let client = PolywrapClient::new(ClientConfig {
        redirects: vec![],
        resolver: Arc::new(BaseResolver::new(
            Box::new(FilesystemResolver::new(Arc::new(file_reader))),
            Box::new(RedirectsResolver::new(redirects)),
        )),
    });

    let invoke_args = InvokeArgs::Msgpack(msgpack!({"a": 1, "b": 2}));

    let invoke_opts = polywrap_core::invoke::InvokeOptions {
        args: Some(&invoke_args),
        env: None,
        resolution_context: None,
        uri: &Uri::from_string("fs/tests/cases/simple-subinvoke/invoke").unwrap(),
        method: "add",
    };

    let invoke_result = client.invoke_and_decode::<String>(&invoke_opts).await.unwrap();

    dbg!(invoke_result);
}
