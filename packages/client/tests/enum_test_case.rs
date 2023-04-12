use polywrap_client::client::PolywrapClient;
use polywrap_client::builder::types::{BuilderConfig, ClientConfigHandler};
use polywrap_client::core::{uri::Uri};
use polywrap_client::msgpack::msgpack;
use polywrap_tests_utils::helpers::get_tests_path;

#[test]
fn enum_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/enum-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    // TODO: Panics instead of returning Result
    let method1a_result = client.invoke::<i32>(
        &uri,
        "method1",
        Some(&msgpack!({
            "en": 5,
        })),
        None,
        None
    );
    let method1a = method1a_result.unwrap_err();
    assert!(method1a.to_string().contains("__wrap_abort: Invalid value for enum 'SanityEnum': 5"));

    let method1b = client.invoke::<i32>(
        &uri,
        "method1",
        Some(&msgpack!({
            "en": 2,
            "optEnum": 1,
        })),
        None,
        None
    ).unwrap();
    assert_eq!(method1b, 2);

    let method1c = client.invoke::<i32>(
        &uri,
        "method1",
        Some(&msgpack!({
            "en": 1,
            "optEnum": "INVALID",
        })),
        None,
        None
    ).unwrap_err();
    assert!(method1c.to_string().contains("__wrap_abort: Invalid key for enum 'SanityEnum': INVALID"));

    let method2a = client.invoke::<Vec<i32>>(
        &uri,
        "method2",
        Some(&msgpack!({
            "enumArray": ["OPTION1", 0, "OPTION3"],
        })),
        None,
        None
    ).unwrap();
    assert_eq!(method2a, vec![0, 0, 2]);
}
