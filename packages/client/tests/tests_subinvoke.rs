use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_client_builder::types::{BuilderConfig, ClientConfigHandler, ClientBuilder};
use polywrap_core::{uri::Uri};
use polywrap_msgpack::msgpack;
use polywrap_tests_utils::helpers::get_tests_path;

#[tokio::test]
async fn subinvoke_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let subinvoke_uri = Uri::try_from(format!("fs/{}/subinvoke/00-subinvoke/implementations/rs", path))
        .unwrap();
    let invoke_uri = Uri::try_from(format!("fs/{}/subinvoke/01-invoke/implementations/rs", path))
        .unwrap();

    let mut builder = BuilderConfig::new(None);
    builder.add_redirect(Uri::try_from("ens/imported-subinvoke.eth").unwrap(), subinvoke_uri.clone());
    let config = builder.build();
    let client = PolywrapClient::new(config);

    let invoke_result = client
        .invoke::<u32>(&invoke_uri, "addAndIncrement", Some(&msgpack!({"a": 1, "b": 1})), None, None)
        .unwrap();

    assert_eq!(invoke_result, 3);
}