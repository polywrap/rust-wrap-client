use polywrap_msgpack_serde::to_vec;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::Serialize;

#[derive(Serialize)]
struct BoolMethodArgs {
    arg: u32,
}

#[derive(Serialize)]
struct IntMethodArgs {
    arg: bool,
}

#[derive(Serialize)]
struct UintMethodArgs {
    arg: Vec<u32>,
}

#[derive(Serialize)]
struct BytesMethodArgs {
    arg: f32,
}

#[derive(Serialize)]
struct ArrayMethodProp {
    prop: String,
}

#[derive(Serialize)]
struct ArrayMethodArgs {
    arg: ArrayMethodProp,
}

use crate::wrap_types::get_client;

#[test]
fn invalid_test_case() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = format!("fs/{}/invalid-type/implementations/rs", path)
        .parse()
        .unwrap();

    let client = get_client(None);

    let invalid_bool_int_sent = client
        .invoke::<bool>(
            &uri,
            "boolMethod",
            Some(&to_vec(&BoolMethodArgs { arg: 10 }).unwrap()),
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
            Some(&to_vec(&IntMethodArgs { arg: true }).unwrap()),
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
            Some(&to_vec(&UintMethodArgs { arg: vec![10] }).unwrap()),
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
            Some(&to_vec(&BytesMethodArgs { arg: 10.15 }).unwrap()),
            None,
            None,
        )
        .unwrap_err();
    assert!(invalid_bytes_float_sent
        .to_string()
        .contains("Property must be of type 'bytes'. Found 'float32'."));

    let invalid_array_map_sent = client
        .invoke::<Vec<i32>>(
            &uri,
            "arrayMethod",
            Some(
                &to_vec(&ArrayMethodArgs {
                    arg: ArrayMethodProp {
                        prop: "".to_string(),
                    },
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap_err();
    assert!(invalid_array_map_sent
        .to_string()
        .contains("Property must be of type 'array'. Found 'map'."));
}
