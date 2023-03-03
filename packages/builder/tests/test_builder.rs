#![feature(closure_lifetime_binder)]
#![feature(trait_upcasting)]
use std::{collections::HashMap};

use polywrap_client_builder::types::{BuilderConfig, ClientBuilder};
use polywrap_core::{
    uri::Uri,
    client::UriRedirect, resolvers::uri_resolution_context::{UriPackage, UriWrapper}
};
use polywrap_tests_utils::helpers::{get_mock_package, MockPackage, MockWrapper, get_mock_wrapper};
use serde_json::json;

#[test]
fn test_env_methods() {
    let mut builder = BuilderConfig::new(None);
    let uri = Uri::new("wrap://ens/wrapper.eth");

    assert!(builder.envs.is_none());

    builder.add_env(uri.clone(), json!({ "d": "d" }));

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.uri);

    assert!(env_from_builder.is_some());
    assert_eq!(env_from_builder.unwrap(), &json!({ "d": "d" }));

    let mut envs = HashMap::new();
    envs.insert(uri.clone().uri, json!({"a": "a", "b": "b"}));

    builder.add_envs(envs);

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.uri);
    assert_eq!(env_from_builder.unwrap(), &json!({ "d": "d", "a": "a", "b": "b" }));

    builder.set_env(uri.clone(), json!({"c": "c"}));

    let current_env = builder.envs.clone().unwrap();
    let env_from_builder = current_env.get(&uri.uri);
    assert_eq!(env_from_builder.unwrap(), &json!({ "c": "c" }));

    builder.remove_env(uri);

    assert!(builder.envs.is_none());
}

#[test]
fn test_interface_implementation_methods() {
    let mut builder = BuilderConfig::new(None);

    let interface_uri = Uri::new("wrap://ens/interface.eth");
    let implementation_a_uri = Uri::new("wrap://ens/implementation-a.eth");
    let implementation_b_uri = Uri::new("wrap://ens/implementation-b.eth");

    assert!(builder.interfaces.is_none());

    builder.add_interface_implementations(
        interface_uri.clone(), 
        vec![implementation_a_uri.clone(), implementation_b_uri.clone()]
    );

    let interfaces = builder.interfaces.clone().unwrap();
    let implementations = interfaces.get(&interface_uri.uri).unwrap();
    assert!(builder.interfaces.is_some());
    assert_eq!(implementations, &vec![implementation_a_uri.clone(), implementation_b_uri.clone()]);

    let implementation_c_uri = Uri::new("wrap://ens/implementation-c.eth");
    builder.add_interface_implementation(interface_uri.clone(), implementation_c_uri.clone());

    let interfaces = builder.interfaces.clone().unwrap();
    let implementations = interfaces.get(&interface_uri.uri).unwrap();
    assert_eq!(implementations, &vec![
        implementation_a_uri.clone(), 
        implementation_b_uri.clone(),
        implementation_c_uri.clone()
    ]);

    builder.remove_interface_implementation(interface_uri.clone(), implementation_b_uri);
    let interfaces = builder.interfaces.clone().unwrap();
    let implementations = interfaces.get(&interface_uri.uri).unwrap();
    assert_eq!(implementations, &vec![
        implementation_a_uri,
        implementation_c_uri
    ]);

}

#[test]
fn test_redirects() {
    let mut builder = BuilderConfig::new(None);
    assert!(!builder.redirects.is_some());

    let redirects = vec![
        UriRedirect{
            from: "ens/c.eth".to_string().try_into().unwrap(), 
            to: "ens/d.eth".to_string().try_into().unwrap()
        },
        UriRedirect{
            from: "ens/f.eth".to_string().try_into().unwrap(), 
            to: "ens/g.eth".to_string().try_into().unwrap()
        },
    ];
    builder.add_redirects(redirects);

    assert!(builder.redirects.is_some());
    let builder_redirects = builder.redirects.unwrap();
    assert_eq!(builder_redirects[0].from, "ens/c.eth".to_string().try_into().unwrap());
    assert_eq!(builder_redirects[0].to, "ens/d.eth".to_string().try_into().unwrap());
    assert_eq!(builder_redirects[1].from, "ens/f.eth".to_string().try_into().unwrap());
    assert_eq!(builder_redirects[1].to, "ens/g.eth".to_string().try_into().unwrap());

    let mut builder = BuilderConfig::new(None);
    assert!(!builder.redirects.is_some());

    builder.add_redirect("ens/a.eth".to_string().try_into().unwrap(), "ens/b.eth".to_string().try_into().unwrap());
    assert!(builder.redirects.is_some());

    builder.remove_redirect("ens/a.eth".to_string().try_into().unwrap());
    assert!(!builder.redirects.is_some());
}

#[test]
fn test_packages() {
    let mut builder = BuilderConfig::new(None);
    assert!(!builder.packages.is_some());

    let uri_package_a = UriPackage{
        uri: String::from("wrap://package/a").try_into().unwrap(),
        package: get_mock_package(Some(String::from("a")))
    };

    let uri_package_b = UriPackage{
        uri: String::from("wrap://package/b").try_into().unwrap(),
        package: get_mock_package(Some(String::from("b")))
    };

    let uri_package_c = UriPackage{
        uri: String::from("wrap://package/c").try_into().unwrap(),
        package: get_mock_package(Some(String::from("c")))
    };

    builder.add_packages(vec![uri_package_a, uri_package_b, uri_package_c]);
    assert!(builder.packages.is_some());
    let builder_packages = builder.packages.unwrap();
    assert_eq!(builder_packages.len(), 3);

    {
        let package_from_builder = &*(builder_packages[1].package.lock().unwrap()) as &dyn std::any::Any;
        let received_package = package_from_builder.downcast_ref::<MockPackage>().unwrap();
        
        let mock_package = get_mock_package(Some(String::from("b")));
        let mock_package_as_any = &*(mock_package.lock().unwrap()) as &dyn std::any::Any;
        let expected_package = mock_package_as_any.downcast_ref::<MockPackage>().unwrap();
        assert_eq!(received_package.name, expected_package.name);
    }

    // We need to recreate the builder because when we do builder.packages.unwrap
    // the ownership is given, not allowing us to call the builder again
    let mut builder = BuilderConfig::new(None);

    let modified_uri_package_b = UriPackage {
        uri: String::from("wrap://package/b").try_into().unwrap(),
        package: get_mock_package(Some(String::from("b-modified")))
    };

    builder.add_packages(builder_packages);
    builder.add_package(modified_uri_package_b);
    builder.remove_package(String::from("wrap://package/c").try_into().unwrap());

    let builder_packages = builder.packages.unwrap();
    assert_eq!(builder_packages.len(), 2);

    let b_package = builder_packages.into_iter().find(|package| package.uri == String::from("wrap://package/b").try_into().unwrap()).unwrap();
    let package_from_builder = &*(b_package.package.lock().unwrap()) as &dyn std::any::Any;
    let received_package = package_from_builder.downcast_ref::<MockPackage>().unwrap();

    let mock_package = get_mock_package(Some(String::from("b-modified")));
    let mock_package_as_any = &*(mock_package.lock().unwrap()) as &dyn std::any::Any;
    let expected_package = mock_package_as_any.downcast_ref::<MockPackage>().unwrap();
    assert_eq!(received_package.name, expected_package.name);
}


#[test]
fn test_wrappers() {
    let mut builder = BuilderConfig::new(None);
    assert!(!builder.wrappers.is_some());

    let uri_wrapper_a = UriWrapper{
        uri: String::from("wrap://wrapper/a").try_into().unwrap(),
        wrapper: get_mock_wrapper(Some(String::from("a")))
    };

    let uri_wrapper_b = UriWrapper{
        uri: String::from("wrap://wrapper/b").try_into().unwrap(),
        wrapper: get_mock_wrapper(Some(String::from("b")))
    };

    let uri_wrapper_c = UriWrapper{
        uri: String::from("wrap://wrapper/c").try_into().unwrap(),
        wrapper: get_mock_wrapper(Some(String::from("c")))
    };

    builder.add_wrappers(vec![uri_wrapper_a, uri_wrapper_b, uri_wrapper_c]);
    assert!(builder.wrappers.is_some());
    let builder_wrappers = builder.wrappers.unwrap();
    assert_eq!(builder_wrappers.len(), 3);

    {
        let wrapper_from_builder = &*(builder_wrappers[1].wrapper.lock().unwrap()) as &dyn std::any::Any;
        let received_wrapper = wrapper_from_builder.downcast_ref::<MockWrapper>().unwrap();
        
        let mock_wrapper = get_mock_wrapper(Some(String::from("b")));
        let mock_wrapper_as_any = &*(mock_wrapper.lock().unwrap()) as &dyn std::any::Any;
        let expected_wrapper = mock_wrapper_as_any.downcast_ref::<MockWrapper>().unwrap();
        assert_eq!(received_wrapper.name, expected_wrapper.name);
    }

    // We need to recreate the builder because when we do builder.wrappers.unwrap
    // the ownership is given, not allowing us to call the builder again
    let mut builder = BuilderConfig::new(None);

    let modified_uri_wrapper_b = UriWrapper {
        uri: String::from("wrap://wrapper/b").try_into().unwrap(),
        wrapper: get_mock_wrapper(Some(String::from("b-modified")))
    };

    builder.add_wrappers(builder_wrappers);
    builder.add_wrapper(modified_uri_wrapper_b);
    builder.remove_wrapper(String::from("wrap://wrapper/c").try_into().unwrap());

    let builder_wrappers = builder.wrappers.unwrap();
    assert_eq!(builder_wrappers.len(), 2);

    let b_wrapper = builder_wrappers.into_iter().find(|wrapper| wrapper.uri == String::from("wrap://wrapper/b").try_into().unwrap()).unwrap();
    let wrapper_from_builder = &*(b_wrapper.wrapper.lock().unwrap()) as &dyn std::any::Any;
    let received_wrapper = wrapper_from_builder.downcast_ref::<MockWrapper>().unwrap();

    let mock_wrapper = get_mock_wrapper(Some(String::from("b-modified")));
    let mock_wrapper_as_any = &*(mock_wrapper.lock().unwrap()) as &dyn std::any::Any;
    let expected_wrapper = mock_wrapper_as_any.downcast_ref::<MockWrapper>().unwrap();
    assert_eq!(received_wrapper.name, expected_wrapper.name);
}