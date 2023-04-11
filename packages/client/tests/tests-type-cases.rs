use std::sync::{Arc, Mutex};
use polywrap_client::client::PolywrapClient;
use polywrap_client::builder::types::{BuilderConfig, ClientConfigHandler, ClientBuilder};
use polywrap_client::core::{uri::Uri};
use polywrap_client::msgpack::msgpack;
use polywrap_core::resolvers::uri_resolution_context::UriPackage;
use polywrap_plugin::package::PluginPackage;
use polywrap_tests_utils::helpers::get_tests_path;
use polywrap_tests_utils::memory_storage_plugin::MemoryStoragePlugin;
use num_bigint::BigInt;
use bigdecimal::BigDecimal as BigNumber;

#[test]
fn asyncify_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/asyncify/implementations/rs", path)).unwrap();

    let memory_storage_plugin = MemoryStoragePlugin { env: serde_json::Value::Null, value: 0 };
    let memory_storage_plugin_package: PluginPackage = memory_storage_plugin.into();
    let memory_storage_package: Arc<Mutex<PluginPackage>> = Arc::new(Mutex::new(memory_storage_plugin_package));
    
    let mut builder = BuilderConfig::new(None);
    builder.add_package(
        UriPackage {
            uri: Uri::try_from("wrap://ens/memory-storage.polywrap.eth".to_string()).unwrap(),
            package: memory_storage_package,
        }
    );
    let config = builder.build();
    let client = PolywrapClient::new(config);

    let subsequent_invokes = client.invoke::<Vec<String>>(
        &uri,
        "subsequentInvokes",
        Some(&msgpack!({"numberOfTimes": 40})),
        None,
        None
    ).unwrap();
    let expected: Vec<String> = (0..40).map(|i| i.to_string()).collect();
    assert_eq!(subsequent_invokes, expected);

    // TODO: panics when args is None
    let local_var_method = client.invoke::<bool>(
        &uri,
        "localVarMethod",
        None,
        None,
        None
    ).unwrap();
    assert_eq!(local_var_method, true);

    let global_var_method = client.invoke::<bool>(
        &uri,
        "global_var_method",
        None,
        None,
        None
    ).unwrap();
    assert_eq!(global_var_method, true);

    let large_str = vec!["polywrap"; 10000].join("");
    let set_data_with_large_args = client.invoke::<String>(
        &uri,
        "setDataWithLargeArgs",
        Some(&msgpack!({"value": large_str.clone()})),
        None,
        None
    ).unwrap();
    assert_eq!(set_data_with_large_args, large_str);

    let set_data_with_many_args = client.invoke::<String>(
        &uri,
        "setDataWithManyArgs",
        Some(&msgpack!({
            "valueA": "polywrap a",
            "valueB": "polywrap b",
            "valueC": "polywrap c",
            "valueD": "polywrap d",
            "valueE": "polywrap e",
            "valueF": "polywrap f",
            "valueG": "polywrap g",
            "valueH": "polywrap h",
            "valueI": "polywrap i",
            "valueJ": "polywrap j",
            "valueK": "polywrap k",
            "valueL": "polywrap l",
        })),
        None,
        None
    ).unwrap();
    let expected = "polywrap apolywrap bpolywrap cpolywrap dpolywrap epolywrap fpolywrap gpolywrap hpolywrap ipolywrap jpolywrap kpolywrap l".to_string();
    assert_eq!(set_data_with_many_args, expected);

    let create_obj = |i: i32| {
        msgpack!({
            "propA": format!("a-{}", i),
            "propB": format!("b-{}", i),
            "propC": format!("c-{}", i),
            "propD": format!("d-{}", i),
            "propE": format!("e-{}", i),
            "propF": format!("f-{}", i),
            "propG": format!("g-{}", i),
            "propH": format!("h-{}", i),
            "propI": format!("i-{}", i),
            "propJ": format!("j-{}", i),
            "propK": format!("k-{}", i),
            "propL": format!("l-{}", i),
        })
    };

    let set_data_with_many_structured_args = client.invoke::<bool>(
        &uri,
        "setDataWithManyStructuredArgs",
        Some(&msgpack!({
            "valueA": create_obj(1),
            "valueB": create_obj(2),
            "valueC": create_obj(3),
            "valueD": create_obj(4),
            "valueE": create_obj(5),
            "valueF": create_obj(6),
            "valueG": create_obj(7),
            "valueH": create_obj(8),
            "valueI": create_obj(9),
            "valueJ": create_obj(10),
            "valueK": create_obj(11),
            "valueL": create_obj(12),
        })),
        None,
        None
    ).unwrap();
    assert_eq!(set_data_with_many_structured_args, true);
}

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

#[test]
fn bignumber_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/bignumber-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    let response_one = client.invoke::<String>(
        &uri,
        "method",
        Some(&msgpack!({
            "arg1": "1234.56789123456789",
            "obj": {
                "prop1": "98.7654321987654321",
            },
        })),
        None,
        None
    ).unwrap();

    let arg1 = "1234.56789123456789".parse::<BigNumber>().unwrap();
    let prop1 = "98.7654321987654321".parse::<BigNumber>().unwrap();
    let result = arg1 * prop1;
    assert_eq!(response_one, result.to_string());

    let response_two = client.invoke::<String>(
        &uri,
        "method",
        Some(&msgpack!({
            "arg1": "1234567.89123456789",
            "arg2": "123456789123.456789123456789123456789",
            "obj": {
                "prop1": "987654.321987654321",
                "prop2": "987.654321987654321987654321987654321",
            },
        })),
        None,
        None
    ).unwrap();

    let arg1 = "1234567.89123456789".parse::<BigNumber>().unwrap();
    let arg2 = "123456789123.456789123456789123456789".parse::<BigNumber>().unwrap();
    let prop1 = "987654.321987654321".parse::<BigNumber>().unwrap();
    let prop2 = "987.654321987654321987654321987654321".parse::<BigNumber>().unwrap();
    let result = arg1 * arg2 * prop1 * prop2;
    assert_eq!(response_two, result.to_string());
}

#[test]
fn bytes_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/bytes-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    // TODO: Panics with invalid return type
    let response = client.invoke::<Vec<u8>>(
        &uri,
        "bytesMethod",
        Some(&msgpack!({
            "arg": {
                "prop": "Argument Value".as_bytes().to_vec(),
            },
        })),
        None,
        None
    ).unwrap();
    let expected = "Argument Value Sanity!".as_bytes().to_vec();
    assert_eq!(response, expected);
}

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

#[test]
fn invalid_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{}/invalid-type/implementations/rs", path)).unwrap();

    let client = PolywrapClient::new(BuilderConfig::new(None).build());

    let invalid_bool_int_sent = client.invoke::<bool>(
        &uri,
        "boolMethod",
        Some(&msgpack!({
            "arg": 10,
        })),
        None,
        None
    ).unwrap_err();
    assert!(invalid_bool_int_sent.to_string().contains("Property must be of type 'bool'. Found 'int'."));

    let invalid_int_bool_sent = client.invoke::<i32>(
        &uri,
        "intMethod",
        Some(&msgpack!({
            "arg": true,
        })),
        None,
        None
    ).unwrap_err();
    assert!(invalid_int_bool_sent.to_string().contains("Property must be of type 'int'. Found 'bool'."));

    let invalid_uint_array_sent = client.invoke::<u32>(
        &uri,
        "uIntMethod",
        Some(&msgpack!({
            "arg": [10],
        })),
        None,
        None
    ).unwrap_err();
    assert!(invalid_uint_array_sent.to_string().contains("Property must be of type 'uint'. Found 'array'."));

    let invalid_bytes_float_sent = client.invoke::<Vec<u8>>(
        &uri,
        "bytesMethod",
        Some(&msgpack!({
            "arg": 10.15,
        })),
        None,
        None
    ).unwrap_err();
    assert!(invalid_bytes_float_sent.to_string().contains("Property must be of type 'bytes'. Found 'float64'."));

    let invalid_array_map_sent = client.invoke::<Vec<i32>>(
        &uri,
        "arrayMethod",
        Some(&msgpack!({
            "arg": {
                "prop": "prop",
            },
        })),
        None,
        None
    ).unwrap_err();
    assert!(invalid_array_map_sent.to_string().contains("Property must be of type 'array'. Found 'map'."));
}