use polywrap_client::polywrap_client::PolywrapClient;
use polywrap_core::{
    client::{ClientConfig, UriRedirect},
    invoke::{InvokeArgs, Invoker}, uri::Uri, env::{Envs, Env}, interface_implementation::InterfaceImplementations,
};
use polywrap_msgpack::{msgpack,Deserialize,Value,decode};

use polywrap_resolvers::{
    base::BaseResolver, filesystem::FilesystemResolver, redirects::RedirectsResolver,
};
use polywrap_core::file_reader::SimpleFileReader;
use polywrap_tests::helpers::get_tests_path;
use std::{sync::Arc, collections::HashMap};


#[tokio::test]
async fn test_env() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let implementation_uri: Uri = format!("fs/{}/interface-invoke/01-implementation/implementations/as", path).try_into().unwrap();
    let wrapper_uri: Uri = format!("fs/{}/interface-invoke/02-wrapper/implementations/as", path).try_into().unwrap();
     
    let mut interfaces: InterfaceImplementations = HashMap::new();
    interfaces.insert("wrap://ens/interface.eth".to_string(), vec![implementation_uri]);


    let file_reader = SimpleFileReader::new();
    let client = PolywrapClient::new(ClientConfig {
        redirects: vec![],
        resolver: Arc::new(BaseResolver::new(
            Box::new(FilesystemResolver::new(Arc::new(file_reader))),
            Box::new(RedirectsResolver::new(vec![])),
        )),
        envs: None,
        interfaces: Some(interfaces)
    });

    let invoke_args = InvokeArgs::Msgpack(msgpack!(
        { 
            "arg": {
                "uint8": 1,
                "str": "Test String 1"
            }
        }
    ));

    let invoke_opts = polywrap_core::invoke::InvokeOptions {
        args: Some(&invoke_args),
        env: None,
        resolution_context: None,
        uri: &wrapper_uri,
        method: "moduleMethod",
    };

    let invoke_result: Vec<u8> = client.invoke(&invoke_opts).await.unwrap();
    dbg!(invoke_result);
}
