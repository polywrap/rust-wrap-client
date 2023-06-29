use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;

use polywrap_client::plugin::Map;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};

use super::get_client;

#[derive(Serialize, Deserialize)]
pub struct ArgsGetKey {
    pub foo: CustomMap,
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct ArgsReturnMap {
    pub map: Map<String, u32>,
}

#[derive(Serialize, Deserialize)]
pub struct ArgsReturnCustomMap {
    pub foo: CustomMap,
}

#[derive(Serialize, Deserialize)]
pub struct ArgsReturnNestedMap {
    pub foo: Map<String, Map<String, u32>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[allow(non_snake_case)]
pub struct CustomMap {
    pub map: Map<String, u32>,
    pub nestedMap: Map<String, Map<String, u32>>,
}

fn get_client_and_uri() -> (PolywrapClient, Uri) {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let uri = Uri::try_from(format!("fs/{path}/map-type/implementations/rs")).unwrap();
    (get_client(None), uri)
}

fn create_custom_map() -> CustomMap {
    let mut my_map = Map::new();
    my_map.insert(String::from("Hello"), 1);
    my_map.insert(String::from("Heyo"), 50);

    let mut my_nested_map = Map::new();
    let mut inside_nested_map = Map::new();

    inside_nested_map.insert(String::from("Hello"), 1);
    inside_nested_map.insert(String::from("Heyo"), 50);
    my_nested_map.insert(String::from("Nested"), inside_nested_map);
    CustomMap {
        map: my_map,
        nestedMap: my_nested_map,
    }
}

#[test]
fn get_key() {
    let (client, uri) = get_client_and_uri();
    let foo = create_custom_map();

    let get_key_args = ArgsGetKey {
        key: String::from("Hello"),
        foo,
    };

    let args = polywrap_msgpack_serde::to_vec(&get_key_args).unwrap();
    let response = client
        .invoke::<u32>(&uri, "getKey", Some(&args), None, None)
        .unwrap();

    assert_eq!(response, 1);
}

#[test]
fn return_map() {
    let (client, uri) = get_client_and_uri();
    let custom_map = create_custom_map();

    let response = client
        .invoke::<Map<String, u32>>(
            &uri,
            "returnMap",
            Some(
                &polywrap_msgpack_serde::to_vec(&ArgsReturnMap {
                    map: custom_map.map.clone(),
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();
    assert_eq!(response, custom_map.map);
}

#[test]
fn return_custom_map() {
    let (client, uri) = get_client_and_uri();
    let custom_map = create_custom_map();

    let response = client
        .invoke::<CustomMap>(
            &uri,
            "returnCustomMap",
            Some(&polywrap_msgpack_serde::to_vec(&ArgsReturnCustomMap { foo: custom_map }).unwrap()),
            None,
            None,
        )
        .unwrap();
    assert_eq!(response, create_custom_map());
}

#[test]
fn return_nested_map() {
    let (client, uri) = get_client_and_uri();
    let mut nested_map: Map<String, Map<String, u32>> = Map::new();
    nested_map.insert(
        String::from("Hello"),
        Map::from([("Nested Hello".to_string(), 59)]),
    );

    nested_map.insert(
        String::from("Bye"),
        Map::from([("Nested Bye".to_string(), 60)]),
    );

    let response = client
        .invoke::<Map<String, Map<String, u32>>>(
            &uri,
            "returnNestedMap",
            Some(
                &polywrap_msgpack_serde::to_vec(&ArgsReturnNestedMap {
                    foo: nested_map.clone(),
                })
                .unwrap(),
            ),
            None,
            None,
        )
        .unwrap();
    assert_eq!(response, nested_map);
}
