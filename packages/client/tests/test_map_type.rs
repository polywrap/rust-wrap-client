use std::collections::BTreeMap;

use polywrap_client::builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler};
use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_client::msgpack::msgpack;
use polywrap_msgpack::extensions::generic_map::GenericMap;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct ArgsGetKey {
    pub key: String,
    pub foo: CustomMap
}

#[derive(Serialize,Deserialize)]
pub struct CustomMap {
    pub map: GenericMap<String, u32>,
    pub nested_map: GenericMap<String, GenericMap<String, u32>>
}


#[test]
fn map_type_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let invoke_uri =
        Uri::try_from(format!("fs/{}/map-type/implementations/rs", path)).unwrap();

    let mut builder = BuilderConfig::new(None);
    let config = builder.build();
    let client = PolywrapClient::new(config);

    let mut myMap = GenericMap(BTreeMap::new());
    myMap.0.insert(String::from("hello"), 1);
    myMap.0.insert(String::from("heyo"), 50);


    let mut myNestedMap = GenericMap(BTreeMap::new());
    let mut insideNestedMap = GenericMap(BTreeMap::new());

    insideNestedMap.0.insert(String::from("hello"), 1);
    insideNestedMap.0.insert(String::from("heyo"), 50);
    myNestedMap.0.insert(String::from("nested"), insideNestedMap);
    let foo = CustomMap {
        map: myMap,
        nested_map: myNestedMap
    };

    let getKeyArgs = ArgsGetKey {
        foo: foo,
        key: String::from("hello")
    };

    let args = rmp_serde::encode::to_vec(&getKeyArgs).unwrap();
    println!("args: {:?}", args);
    // let args = polywrap_msgpack::encode(&getKeyArgs).unwrap();

    let invoke_result = client
        .invoke::<u32>(
            &invoke_uri,
            "getKey",
            Some(&args),
            None,
            None,
        )
        .unwrap();

    assert_eq!(invoke_result, 1);
}
