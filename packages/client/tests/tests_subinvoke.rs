use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_core::{client::UriRedirect, invoke::InvokeArgs, uri::Uri};
use polywrap_msgpack::msgpack;
use polywrap_core::resolvers::{
    static_resolver::{StaticResolver, StaticResolverLike},
};
use polywrap_resolvers::legacy::{base::BaseResolver, filesystem::FilesystemResolver};
use polywrap_tests_utils::helpers::get_tests_path;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn subinvoke_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let subinvoke_uri: Uri = format!("fs/{}/subinvoke/00-subinvoke/implementations/as", path)
        .try_into()
        .unwrap();
    let invoke_uri: Uri = format!("fs/{}/subinvoke/01-invoke/implementations/as", path)
        .try_into()
        .unwrap();

    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

    let redirect = UriRedirect::new("ens/add.eth".try_into().unwrap(), subinvoke_uri.clone());

    let redirects_static_like = StaticResolverLike::Redirect(redirect);
    let static_resolver = StaticResolver::from(vec![
        redirects_static_like
    ]);

    let client = PolywrapClient::new(
        ClientConfig {
            envs: None,
            interfaces: None,
            resolver: Arc::new(BaseResolver::new(
                Box::new(fs_resolver),
                Box::new(static_resolver),
            )),
        }
    );

    let invoke_args = InvokeArgs::Msgpack(msgpack!({"a": 1, "b": 1}));

    let invoke_result = client
        .invoke_and_decode::<u32>(
            &invoke_uri,
            "addAndIncrement",
            Some(&invoke_args),
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(invoke_result, 3);
}
