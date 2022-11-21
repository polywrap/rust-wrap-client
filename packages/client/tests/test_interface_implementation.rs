use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::{
    interface_implementation::InterfaceImplementations,
    invoke::{InvokeArgs, Invoker},
    uri::Uri,
};
use polywrap_msgpack::msgpack;

use polywrap_core::file_reader::SimpleFileReader;
use polywrap_resolvers::{
    base::BaseResolver, filesystem::FilesystemResolver, static_::static_resolver::StaticResolver,
};
use polywrap_tests_utils::helpers::get_tests_path;
use std::{collections::HashMap, sync::Arc};

#[tokio::test]
async fn test_env() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let implementation_uri: Uri = format!(
        "fs/{}/interface-invoke/01-implementation/implementations/as",
        path
    )
    .try_into()
    .unwrap();
    let wrapper_uri: Uri = format!("fs/{}/interface-invoke/02-wrapper/implementations/as", path)
        .try_into()
        .unwrap();

    let mut interfaces: InterfaceImplementations = HashMap::new();
    interfaces.insert(
        "wrap://ens/interface.eth".to_string(),
        vec![implementation_uri],
    );

    let static_resolver = StaticResolver::from(vec![]);

    let file_reader = SimpleFileReader::new();
    let client = PolywrapClient::new(
        Box::new(BaseResolver::new(
            Box::new(FilesystemResolver::new(Arc::new(file_reader))),
            Box::new(static_resolver),
        )),
        Some(interfaces),
    );

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
    dbg!(invoke_result);
}
