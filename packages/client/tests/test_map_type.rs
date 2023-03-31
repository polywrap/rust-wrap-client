use std::collections::BTreeMap;

use polywrap_client::builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler};
use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;
use polywrap_client::msgpack::msgpack;
use polywrap_msgpack::Value;
use polywrap_msgpack::extensions::generic_map::GenericMap;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize)]
pub struct ArgsGetKey {
    pub foo: CustomMap,
    pub key: String,
}

#[derive(Serialize,Deserialize)]
pub struct CustomMap {
    pub map: GenericMap<String, u32>,
    pub nestedMap: GenericMap<String, GenericMap<String, u32>>
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
    myMap.0.insert(String::from("Hello"), 1);
    myMap.0.insert(String::from("Heyo"), 50);


    let mut myNestedMap = GenericMap(BTreeMap::new());
    let mut insideNestedMap = GenericMap(BTreeMap::new());

    insideNestedMap.0.insert(String::from("Hello"), 1);
    insideNestedMap.0.insert(String::from("Heyo"), 50);
    myNestedMap.0.insert(String::from("Nested"), insideNestedMap);
    let foo = CustomMap {
        map: myMap,
        nestedMap: myNestedMap
    };

    let getKeyArgs = ArgsGetKey {
      key: String::from("Hello"),
      foo: foo,
    };

    let args = polywrap_msgpack::serialize(&getKeyArgs).unwrap();
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
