use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_client_builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler};
use polywrap_core::{
    interface_implementation::InterfaceImplementations,
    invoke::{InvokeArgs, Invoker},
    uri::Uri,
};
use polywrap_msgpack::msgpack;

use polywrap_tests_utils::helpers::get_tests_path;
use std::{collections::HashMap};



#[tokio::test]
async fn test_env() {
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

    let invoke_args = InvokeArgs::Msgpack(msgpack!(
        {
            "arg": {
                "uint8": 1,
                "str": "Test String 1"
            }
        }
    ));

    let invoke_result: Vec<u8> = client
        .invoke(&wrapper_uri, "moduleMethod", Some(&invoke_args), None, None)
        .await
        .unwrap();
}
