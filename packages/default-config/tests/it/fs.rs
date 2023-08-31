use polywrap_client::client::PolywrapClient;
use polywrap_client_default_config::SystemClientConfig;
use polywrap_msgpack_serde::to_vec;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::Serialize;

#[derive(Serialize)]
pub struct ArgsAdd {
    pub a: u32,
    pub b: u32,
}

#[test]
fn sanity() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let subinvoke_wrap_uri = format!("fs/{path}/subinvoke/00-subinvoke/implementations/rs");

    let client = PolywrapClient::new(SystemClientConfig::precompiled().into());

    let result = client
        .invoke::<u32>(
            &subinvoke_wrap_uri.parse().unwrap(),
            "add",
            Some(&to_vec(&ArgsAdd { a: 2, b: 40 }).unwrap()),
            None,
            None,
        )
        .unwrap();

    assert_eq!(result, 42);
}
