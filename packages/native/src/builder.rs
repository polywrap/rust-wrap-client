use std::sync::{Arc, Mutex};

use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::uri::Uri,
};
use polywrap_client_default_config::{SystemClientConfig, Web3ClientConfig};

use crate::{
    client::FFIClient,
    package::{FFIWrapPackage, WrapPackageWrapping},
    resolvers::ffi_resolver::{FFIUriResolver, UriResolverWrapping},
    uri::FFIUri,
    wrapper::{FFIWrapper, WrapperWrapping},
};

pub struct FFIBuilderConfig(Mutex<PolywrapClientConfig>);

impl FFIBuilderConfig {
    pub fn new() -> FFIBuilderConfig {
        FFIBuilderConfig(Mutex::new(PolywrapClientConfig::new()))
    }

    pub fn add_env(&self, uri: Arc<FFIUri>, env: Vec<u8>) {
        self.0
            .lock()
            .unwrap()
            .add_env(uri.0.clone(), env);
    }

    pub fn remove_env(&self, uri: Arc<FFIUri>) {
        self.0.lock().unwrap().remove_env(&uri.0);
    }

    pub fn add_interface_implementations(
        &self,
        interface_uri: Arc<FFIUri>,
        implementation_uris: Vec<Arc<FFIUri>>,
    ) {
        let implementations = implementation_uris
            .clone()
            .iter()
            .map(|i| i.0.clone())
            .collect::<Vec<Uri>>();
        self.0
            .lock()
            .unwrap()
            .add_interface_implementations(interface_uri.0.clone(), implementations);
    }

    pub fn add_interface_implementation(
        &self,
        interface_uri: Arc<FFIUri>,
        implementation_uri: Arc<FFIUri>,
    ) {
        self.0
            .lock()
            .unwrap()
            .add_interface_implementation(interface_uri.0.clone(), implementation_uri.0.clone());
    }

    pub fn remove_interface_implementation(
        &self,
        interface_uri: Arc<FFIUri>,
        implementation_uri: Arc<FFIUri>,
    ) {
        self.0
            .lock()
            .unwrap()
            .remove_interface_implementation(&interface_uri.0, &implementation_uri.0);
    }

    pub fn add_wrapper(&self, uri: Arc<FFIUri>, wrapper: Box<dyn FFIWrapper>) {
        self.0
            .lock()
            .unwrap()
            .add_wrapper(uri.0.clone(), Arc::new(WrapperWrapping(wrapper)));
    }

    pub fn remove_wrapper(&self, uri: Arc<FFIUri>) {
        self.0.lock().unwrap().remove_wrapper(&uri.0);
    }

    pub fn add_package(&self, uri: Arc<FFIUri>, package: Box<dyn FFIWrapPackage>) {
        self.0
            .lock()
            .unwrap()
            .add_package(uri.0.clone(), Arc::new(WrapPackageWrapping(package)));
    }

    pub fn remove_package(&self, uri: Arc<FFIUri>) {
        self.0.lock().unwrap().remove_package(&uri.0);
    }

    pub fn add_redirect(&self, from: Arc<FFIUri>, to: Arc<FFIUri>) {
        self.0
            .lock()
            .unwrap()
            .add_redirect(from.0.clone(), to.0.clone());
    }

    pub fn remove_redirect(&self, from: Arc<FFIUri>) {
        self.0.lock().unwrap().remove_redirect(&from.0);
    }

    pub fn add_resolver(&self, resolver: Box<dyn FFIUriResolver>) {
        self.0
            .lock()
            .unwrap()
            .add_resolver(Arc::from(UriResolverWrapping(resolver).as_uri_resolver()));
    }

    pub fn add_system_defaults(&self) {
        self.0
            .lock()
            .unwrap()
            .add(SystemClientConfig::default().into())
            .add(Web3ClientConfig::default().into());
    }

    pub fn add_web3_defaults(&self) {
      self.0
            .lock()
            .unwrap()
            .add(Web3ClientConfig::default().into());
    }

    pub fn build(&self) -> Arc<FFIClient> {
        let config = self.0.lock().unwrap().clone();
        let client = Arc::new(PolywrapClient::new(config.into()));
        Arc::new(FFIClient::new(client))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use polywrap_client::core::{macros::uri, uri::Uri};
    use polywrap_msgpack_serde::to_vec;
    use polywrap_tests_utils::mocks::{
        get_different_mock_package, get_different_mock_wrapper, get_mock_package, get_mock_wrapper,
    };
    use serde::Serialize;

    use crate::{package::FFIWrapPackage, uri::ffi_uri_from_string, wrapper::FFIWrapper};

    use super::FFIBuilderConfig;

    #[derive(Serialize)]
    struct Args {
        foo: String,
    }

    #[derive(Serialize)]
    struct AnotherArgs {
        bar: String,
    }

    #[test]
    fn adds_and_removes_env() {
        let builder = FFIBuilderConfig::new();
        let uri = ffi_uri_from_string("wrap://ens/some.eth").unwrap();
        let env = to_vec(&Args {
            foo: "bar".to_string(),
        })
        .unwrap();

        builder.add_env(uri.clone(), env.clone());

        let envs = builder.0.lock().unwrap().clone().envs.unwrap();
        let current_env = envs.get(&uri!("wrap://ens/some.eth"));
        assert_eq!(&env, current_env.unwrap());

        let new_env = to_vec(&AnotherArgs {
            bar: "foo".to_string(),
        })
        .unwrap();
        builder.add_env(uri.clone(), new_env.clone());
        let envs = builder.0.lock().unwrap().clone().envs.unwrap();
        let current_env = envs.get(&uri!("wrap://ens/some.eth"));
        assert_eq!(&new_env, current_env.unwrap());

        builder.remove_env(uri);
        let envs = builder.0.lock().unwrap().clone().envs;
        assert_eq!(None, envs);
    }

    #[test]
    fn adds_and_removes_package() {
        let builder = FFIBuilderConfig::new();
        let mock_package: Box<dyn FFIWrapPackage> = Box::new(get_mock_package());
        let uri_mock_package = ffi_uri_from_string("package/a").unwrap();
        let different_mock_package: Box<dyn FFIWrapPackage> =
            Box::new(get_different_mock_package());
        let uri_different_mock_package = ffi_uri_from_string("package/b").unwrap();

        builder.add_package(uri_mock_package.clone(), mock_package);
        builder.add_package(uri_different_mock_package, different_mock_package);
        assert_eq!(
            builder
                .0
                .lock()
                .unwrap()
                .clone()
                .packages
                .unwrap()
                .len(),
            2
        );

        builder.remove_package(uri_mock_package);
        assert_eq!(
            builder
                .0
                .lock()
                .unwrap()
                .clone()
                .packages
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn adds_and_removes_wrapper() {
        let builder = FFIBuilderConfig::new();
        let mock_package: Box<dyn FFIWrapper> = Box::new(get_mock_wrapper());
        let uri_mock_wrapper = ffi_uri_from_string("wrap/a");
        let different_mock_wrapper: Box<dyn FFIWrapper> = Box::new(get_different_mock_wrapper());
        let uri_different_mock_wrapper = ffi_uri_from_string("wrap/b");

        builder.add_wrapper(uri_mock_wrapper.clone().unwrap(), mock_package);
        builder.add_wrapper(uri_different_mock_wrapper.unwrap(), different_mock_wrapper);
        assert_eq!(
            builder
                .0
                .lock()
                .unwrap()
                .clone()
                .wrappers
                .unwrap()
                .len(),
            2
        );

        builder.remove_wrapper(uri_mock_wrapper.unwrap());
        assert_eq!(
            builder
                .0
                .lock()
                .unwrap()
                .clone()
                .wrappers
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn adds_and_removes_redirects() {
        let builder = FFIBuilderConfig::new();
        builder.add_redirect(
            ffi_uri_from_string("wrap/a").unwrap(),
            ffi_uri_from_string("wrap/b").unwrap(),
        );
        builder.add_redirect(
            ffi_uri_from_string("wrap/c").unwrap(),
            ffi_uri_from_string("wrap/d").unwrap(),
        );

        let redirects = builder
            .0
            .lock()
            .unwrap()
            .clone()
            .redirects
            .unwrap();
        assert_eq!(
            redirects,
            HashMap::from([
                (uri!("wrap/a"), uri!("wrap/b")),
                (uri!("wrap/c"), uri!("wrap/d")),
            ])
        );
    }

    #[test]
    fn adds_and_removes_interface_implementation() {
        let builder = FFIBuilderConfig::new();
        let interface_uri = ffi_uri_from_string("wrap://ens/interface.eth").unwrap();
        let implementation_a_uri = ffi_uri_from_string("wrap://ens/implementation-a.eth").unwrap();
        let implementation_b_uri = ffi_uri_from_string("wrap://ens/implementation-b.eth").unwrap();

        let implementation_c_uri = ffi_uri_from_string("wrap://ens/implementation-c.eth").unwrap();
        builder.add_interface_implementations(
            interface_uri.clone(),
            vec![implementation_a_uri, implementation_b_uri.clone()],
        );
        builder.add_interface_implementation(interface_uri.clone(), implementation_c_uri.clone());

        let interfaces: HashMap<String, Vec<polywrap_client::core::uri::Uri>> = builder
            .0
            .lock()
            .unwrap()
            .clone()
            .interfaces
            .unwrap();
        let implementations = interfaces.get(&interface_uri.to_string());
        assert_eq!(
            implementations,
            Some(&vec![
                uri!("wrap://ens/implementation-a.eth"),
                uri!("wrap://ens/implementation-b.eth"),
                uri!("wrap://ens/implementation-c.eth")
            ])
        );

        builder.remove_interface_implementation(interface_uri.clone(), implementation_b_uri);
        let interfaces: HashMap<String, Vec<polywrap_client::core::uri::Uri>> = builder
            .0
            .lock()
            .unwrap()
            .clone()
            .interfaces
            .unwrap();
        let implementations = interfaces.get(&interface_uri.to_string());
        assert_eq!(
            implementations,
            Some(&vec![
                uri!("wrap://ens/implementation-a.eth"),
                uri!("wrap://ens/implementation-c.eth")
            ])
        );
    }

    #[test]
    fn add_system_defaults() {
        let builder = FFIBuilderConfig::new();
        builder.add_system_defaults();
        assert!(builder.0.lock().unwrap().redirects.is_some());
        assert!(builder.0.lock().unwrap().interfaces.is_some());
        assert!(builder.0.lock().unwrap().wrappers.is_some());
        assert!(builder.0.lock().unwrap().packages.is_some());
    }

    #[test]
    fn add_web3_defaults() {
        let builder = FFIBuilderConfig::new();
        builder.add_web3_defaults();
        assert!(builder.0.lock().unwrap().envs.is_some());
        assert!(builder.0.lock().unwrap().interfaces.is_some());
        assert!(builder.0.lock().unwrap().wrappers.is_some());
    }
}
