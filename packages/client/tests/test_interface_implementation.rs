use polywrap_client::client::PolywrapClient;
use polywrap_client::builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler};
use polywrap_client::core::{
    interface_implementation::InterfaceImplementations,
    uri::Uri,
};
use polywrap_client::msgpack::msgpack;

use polywrap_tests_utils::helpers::get_tests_path;
use serde::Deserialize;
use std::{collections::HashMap};

#[derive(Deserialize, Debug, PartialEq)]
struct ModuleMethodResponse {
    uint8: i8,
    str: String
}

#[test]
fn test_interface_implementation() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let implementation_uri = Uri::try_from(format!(
        "fs/{}/interface-invoke/01-implementation/implementations/as",
        path
    )).unwrap();
    let wrapper_uri = Uri::try_from(format!(
        "fs/{}/interface-invoke/02-wrapper/implementations/as", path
    )).unwrap();

    let mut interfaces: InterfaceImplementations = HashMap::new();
    interfaces.insert(
        "wrap://ens/interface.eth".to_string(),
        vec![implementation_uri.clone()],
    );
    let mut builder = BuilderConfig::new(None);
    builder.add_interface_implementation(
        Uri::try_from("wrap://ens/interface.eth").unwrap(),
        implementation_uri
    );
    let config = builder.build();

    let client = PolywrapClient::new(config);

    let invoke_result = client
        .invoke::<ModuleMethodResponse>(&wrapper_uri, "moduleMethod", Some(&msgpack!(
          {
              "arg": {
                  "uint8": 1,
                  "str": "Test String 1"
              }
          }
      )), None, None)
        .unwrap();
    let mock_response = ModuleMethodResponse {
        uint8: 1,
        str: "Test String 1".to_string()
    };
    assert_eq!(invoke_result, mock_response);
}
