use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_client::msgpack::msgpack;
use polywrap_client_builder::PolywrapClientConfig;
use polywrap_tests_utils::helpers::get_tests_path;

#[test]
#[ignore]
fn invalid_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/invalid-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(PolywrapClientConfig::new().into());

    let invalid_bool_int_sent = client
        .invoke::<bool>(
            &uri,
            "boolMethod",
            Some(&msgpack!({
                "arg": 10,
            })),
            None,
            None,
        )
        .unwrap_err();
    assert!(invalid_bool_int_sent
        .to_string()
        .contains("Property must be of type 'bool'. Found 'int'."));

    let invalid_int_bool_sent = client
        .invoke::<i32>(
            &uri,
            "intMethod",
            Some(&msgpack!({
                "arg": true,
            })),
            None,
            None,
        )
        .unwrap_err();
    assert!(invalid_int_bool_sent
        .to_string()
        .contains("Property must be of type 'int'. Found 'bool'."));

    let invalid_uint_array_sent = client
        .invoke::<u32>(
            &uri,
            "uIntMethod",
            Some(&msgpack!({
                "arg": [10],
            })),
            None,
            None,
        )
        .unwrap_err();
    assert!(invalid_uint_array_sent
        .to_string()
        .contains("Property must be of type 'uint'. Found 'array'."));

    let invalid_bytes_float_sent = client
        .invoke::<Vec<u8>>(
            &uri,
            "bytesMethod",
            Some(&msgpack!({
                "arg": 10.15,
            })),
            None,
            None,
        )
        .unwrap_err();
    assert!(invalid_bytes_float_sent
        .to_string()
        .contains("Property must be of type 'bytes'. Found 'float64'."));

    let invalid_array_map_sent = client
        .invoke::<Vec<i32>>(
            &uri,
            "arrayMethod",
            Some(&msgpack!({
                "arg": {
                    "prop": "prop",
                },
            })),
            None,
            None,
        )
        .unwrap_err();
    assert!(invalid_array_map_sent
        .to_string()
        .contains("Property must be of type 'array'. Found 'map'."));
}
