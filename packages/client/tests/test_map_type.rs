use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use polywrap_client::client::PolywrapClient;
use polywrap_client::core::uri::Uri;

use polywrap_client::plugin::Map;
use polywrap_core::client::ClientConfig;
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_resolvers::base_resolver::BaseResolver;
use polywrap_resolvers::simple_file_resolver::FilesystemResolver;
use polywrap_resolvers::static_resolver::StaticResolver;
use polywrap_tests_utils::helpers::get_tests_path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArgsGetKey {
    pub bar: CustomMap,
    pub key: String,
}

#[derive(Serialize, Deserialize)]
pub struct CustomMap {
    pub map: Map<String, u32>,
    pub nestedMap: Map<String, Map<String, u32>>,
}

#[test]
fn map_type_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let invoke_uri = Uri::try_from(format!("fs/{path}/map-type/implementations/rs")).unwrap();

    let file_reader = SimpleFileReader::new();
    let fs_resolver = FilesystemResolver::new(Arc::new(file_reader));

    let base_resolver = BaseResolver::new(
        Box::new(fs_resolver),
        Box::new(StaticResolver::new(HashMap::new())),
    );
    let config = ClientConfig {
        envs: None,
        resolver: Arc::new(base_resolver),
        interfaces: None,
    };
    let client = PolywrapClient::new(config);
    let mut my_map = Map(BTreeMap::new());
    my_map.0.insert(String::from("Hello"), 1);
    my_map.0.insert(String::from("Heyo"), 50);

    let mut my_nested_map = Map(BTreeMap::new());
    let mut inside_nested_map = Map(BTreeMap::new());

    inside_nested_map.0.insert(String::from("Hello"), 1);
    inside_nested_map.0.insert(String::from("Heyo"), 50);
    my_nested_map
        .0
        .insert(String::from("Nested"), inside_nested_map);
    let bar = CustomMap {
        map: my_map,
        nestedMap: my_nested_map,
    };

    let get_key_args = ArgsGetKey {
        key: String::from("Hello"),
        bar,
    };

    let args = polywrap_msgpack::serialize(&get_key_args).unwrap();
    let invoke_result = client
        .invoke::<u32>(&invoke_uri, "getKey", Some(&args), None, None)
        .unwrap();

    assert_eq!(invoke_result, 1);
}
