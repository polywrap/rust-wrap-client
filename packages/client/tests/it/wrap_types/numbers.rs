use polywrap_client::builder::types::{BuilderConfig, ClientConfigHandler};
use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_client::msgpack::msgpack;
use polywrap_tests_utils::helpers::get_tests_path;

#[test]
#[ignore]
fn numbers_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/numbers-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    let i8_underflow = client
        .invoke::<i8>(
            &uri,
            "i8Method",
            Some(&msgpack!({
                "first": -129,
                "second": 10,
            })),
            None,
            None,
        )
        .unwrap_err();
    assert!(i8_underflow
        .to_string()
        .contains("integer overflow: value = -129; bits = 8"));

    let u8_overflow = client
        .invoke::<u8>(
            &uri,
            "u8Method",
            Some(&msgpack!({
                "first": 256,
                "second": 10,
            })),
            None,
            None,
        )
        .unwrap_err();
    assert!(u8_overflow
        .to_string()
        .contains("unsigned integer overflow: value = 256; bits = 8"));

    let i16_underflow = client
        .invoke::<i16>(
            &uri,
            "i16Method",
            Some(&msgpack!({
                "first": -32769,
                "second": 10,
            })),
            None,
            None,
        )
        .unwrap_err();
    assert!(i16_underflow
        .to_string()
        .contains("integer overflow: value = -32769; bits = 16"));

    let u16_overflow = client
        .invoke::<u16>(
            &uri,
            "u16Method",
            Some(&msgpack!({
                "first": 65536,
                "second": 10,
            })),
            None,
            None,
        )
        .unwrap_err();
    assert!(u16_overflow
        .to_string()
        .contains("unsigned integer overflow: value = 65536; bits = 16"));

    let i32_underflow = client
        .invoke::<i32>(
            &uri,
            "i32Method",
            Some(&msgpack!({
                "first": -2147483649i64,
                "second": 10,
            })),
            None,
            None,
        )
        .unwrap_err();
    assert!(i32_underflow
        .to_string()
        .contains("integer overflow: value = -2147483649; bits = 32"));

    let u32_overflow = client
        .invoke::<u32>(
            &uri,
            "u32Method",
            Some(&msgpack!({
                "first": 4294967296u64,
                "second": 10,
            })),
            None,
            None,
        )
        .unwrap_err();
    assert!(u32_overflow
        .to_string()
        .contains("unsigned integer overflow: value = 4294967296; bits = 32"));
}
