use polywrap_client::client::PolywrapClient;
use polywrap_client::builder::types::{BuilderConfig, ClientConfigHandler};
use polywrap_client::core::{uri::Uri};
use polywrap_client::msgpack::msgpack;
use polywrap_tests_utils::helpers::get_tests_path;
use num_bigint::BigInt;

#[test]
fn bigint_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/bigint-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    let response_one = client.invoke::<String>(
        &uri,
        "method",
        Some(&msgpack!({
            "arg1": "123456789123456789",
            "obj": {
                "prop1": "987654321987654321",
            },
        })),
        None,
        None
    ).unwrap();
    let expected = "123456789123456789".parse::<BigInt>().unwrap() * "987654321987654321".parse::<BigInt>().unwrap();
    assert_eq!(response_one, expected.to_string());

    let response = client.invoke::<String>(
        &uri,
        "method",
        Some(&msgpack!({
            "arg1": "123456789123456789",
            "arg2": "123456789123456789123456789123456789",
            "obj": {
                "prop1": "987654321987654321",
                "prop2": "987654321987654321987654321987654321",
            },
        })),
        None,
        None
    ).unwrap();

    let expected = "123456789123456789".parse::<BigInt>().unwrap() *
        "123456789123456789123456789123456789".parse::<BigInt>().unwrap() *
        "987654321987654321".parse::<BigInt>().unwrap() *
        "987654321987654321987654321987654321".parse::<BigInt>().unwrap();
    assert_eq!(response, expected.to_string());
}