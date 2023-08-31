use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::Client,
    core::uri::Uri,
};
use polywrap_client_default_config::{SystemClientConfig, Web3ClientConfig};

use crate::{
    client::FFIClient,
    package::FFIWrapPackage,
    resolvers::{
        ffi_resolver::FFIUriResolver,
        uri_package_or_wrapper::{FFIUriWrapPackage, FFIUriWrapper},
    },
    uri::FFIUri,
    wrapper::FFIWrapper,
};

pub struct FFIBuilderConfig(Mutex<PolywrapClientConfig>);

impl FFIBuilderConfig {
    pub fn new() -> FFIBuilderConfig {
        FFIBuilderConfig(Mutex::new(PolywrapClientConfig::new()))
    }

    pub fn get_interfaces(&self) -> Option<HashMap<String, Vec<Arc<FFIUri>>>> {
        let config = self.0.try_lock().unwrap();

        config.interfaces.clone().map(|interfaces| {
            interfaces
                .into_iter()
                .map(|(uri, value)| (uri.to_string(), value.into_iter().map(|uri| Arc::new(FFIUri(uri))).collect()))
                .collect()
        })
    }

    pub fn get_envs(&self) -> Option<HashMap<String, Vec<u8>>> {
        let config = self.0.try_lock().unwrap();

        config.envs.clone().map(|envs| {
            envs.into_iter()
                .map(|(uri, value)| (uri.to_string(), value))
                .collect()
        })
    }

    pub fn get_wrappers(&self) -> Option<Vec<FFIUriWrapper>> {
        let config = self.0.try_lock().unwrap();

        config.wrappers.clone().map(|wrappers| {
            wrappers
                .into_iter()
                .map(|(uri, wrapper)| {
                    FFIUriWrapper::new(Arc::new(FFIUri(uri)), Box::new(wrapper))
                })
                .collect()
        })
    }

    pub fn get_packages(&self) -> Option<Vec<FFIUriWrapPackage>> {
        let config = self.0.try_lock().unwrap();

        config.packages.clone().map(|packages| {
            packages
                .into_iter()
                .map(|(uri, package)| {
                    FFIUriWrapPackage::new(
                        Arc::new(FFIUri(uri)),
                        Box::new(package),
                    )
                })
                .collect()
        })
    }

    pub fn get_redirects(&self) -> Option<HashMap<String, Arc<FFIUri>>> {
        let config = self.0.try_lock().unwrap();

        config.redirects.clone().map(|redirects| {
            redirects
                .into_iter()
                .map(|redirect| (redirect.0.to_string(), Arc::new(FFIUri(redirect.1))))
                .collect()
        })
    }

    pub fn get_resolvers(&self) -> Option<Vec<Arc<FFIUriResolver>>> {
        let config = self.0.try_lock().unwrap();

        config.resolvers.clone().map(|resolvers| {
            resolvers
                .into_iter()
                .map(|resolver| Arc::new(FFIUriResolver(Box::new(resolver))))
                .collect()
        })
    }

    pub fn add_env(&self, uri: Arc<FFIUri>, env: Vec<u8>) {
        self.0.lock().unwrap().add_env(uri.0.clone(), env);
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

    pub fn add_wrapper(&self, uri: Arc<FFIUri>, wrapper: Arc<FFIWrapper>) {
        self.0.lock().unwrap().add_wrapper(uri.0.clone(), wrapper);
    }

    pub fn remove_wrapper(&self, uri: Arc<FFIUri>) {
        self.0.lock().unwrap().remove_wrapper(&uri.0);
    }

    pub fn add_package(&self, uri: Arc<FFIUri>, package: Arc<FFIWrapPackage>) {
        self.0.lock().unwrap().add_package(uri.0.clone(), package);
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

    pub fn add_resolver(&self, resolver: Arc<FFIUriResolver>) {
        self.0.lock().unwrap().add_resolver(resolver);
    }

    pub fn add_system_defaults(&self) {
        self.0
            .lock()
            .unwrap()
            .add(SystemClientConfig::default().into());
    }

    pub fn add_web3_defaults(&self) {
        self.0
            .lock()
            .unwrap()
            .add(Web3ClientConfig::default().into());
    }

    pub fn build(&self) -> Arc<FFIClient> {
        let config = self.0.lock().unwrap().clone();
        let client = Arc::new(Client::new(config.into()));
        Arc::new(FFIClient::new(client))
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, sync::Arc};

    use polywrap_client::core::{macros::uri, uri::Uri};
    use polywrap_msgpack_serde::to_vec;
    use polywrap_tests_utils::mocks::{
        get_different_mock_package, get_different_mock_wrapper, get_mock_package, get_mock_wrapper,
    };
    use serde::Serialize;

    use crate::{package::FFIWrapPackage, uri::{ffi_uri_from_string, FFIUri}, wrapper::FFIWrapper, resolvers::{_static::FFIStaticUriResolver, ffi_resolver::FFIUriResolver}};

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
        let uri = ffi_uri_from_string("wrap://mock/some").unwrap();
        let env = to_vec(&Args {
            foo: "bar".to_string(),
        })
        .unwrap();

        builder.add_env(uri.clone(), env.clone());

        let envs = builder.0.lock().unwrap().clone().envs.unwrap();
        let current_env = envs.get(&uri!("wrap://mock/some"));
        assert_eq!(&env, current_env.unwrap());
        assert_eq!(builder.get_envs().unwrap(), HashMap::from([
            (
                "wrap://mock/some".to_string(),
                env.clone()
            )
        ]));

        let new_env = to_vec(&AnotherArgs {
            bar: "foo".to_string(),
        })
        .unwrap();
        builder.add_env(uri.clone(), new_env.clone());

        let envs = builder.0.lock().unwrap().clone().envs.unwrap();
        let current_env = envs.get(&uri!("wrap://mock/some"));
        assert_eq!(&new_env, current_env.unwrap());
        assert_eq!(builder.get_envs().unwrap(), HashMap::from([
            (
                "wrap://mock/some".to_string(),
                new_env.clone()
            )
        ]));

        builder.remove_env(uri);
        let envs = builder.0.lock().unwrap().clone().envs;
        assert_eq!(None, envs);
        assert_eq!(builder.get_envs(), None);
    }

    #[test]
    fn adds_and_removes_package() {
        let builder = FFIBuilderConfig::new();
        let mock_package = Arc::new(FFIWrapPackage(Box::new(get_mock_package())));
        let uri_mock_package = ffi_uri_from_string("package/a").unwrap();
        let different_mock_package =
            Arc::new(FFIWrapPackage(Box::new(get_different_mock_package())));
        let uri_different_mock_package = ffi_uri_from_string("package/b").unwrap();

        builder.add_package(uri_mock_package.clone(), mock_package);
        builder.add_package(uri_different_mock_package, different_mock_package);
        assert_eq!(builder.0.lock().unwrap().clone().packages.unwrap().len(), 2);
        assert_eq!(builder.get_packages().unwrap().len(), 2);

        builder.remove_package(uri_mock_package);
        assert_eq!(builder.0.lock().unwrap().clone().packages.unwrap().len(), 1);
        assert_eq!(builder.get_packages().unwrap().len(), 1);
    }

    #[test]
    fn adds_and_removes_wrapper() {
        let builder = FFIBuilderConfig::new();
        let mock_package = Arc::new(FFIWrapper(Box::new(get_mock_wrapper())));
        let uri_mock_wrapper = ffi_uri_from_string("wrap/a");
        let different_mock_wrapper = Arc::new(FFIWrapper(Box::new(get_different_mock_wrapper())));
        let uri_different_mock_wrapper = ffi_uri_from_string("wrap/b");

        builder.add_wrapper(uri_mock_wrapper.clone().unwrap(), mock_package);
        builder.add_wrapper(uri_different_mock_wrapper.unwrap(), different_mock_wrapper);
        assert_eq!(builder.0.lock().unwrap().clone().wrappers.unwrap().len(), 2);
        assert_eq!(builder.get_wrappers().unwrap().len(), 2);

        builder.remove_wrapper(uri_mock_wrapper.unwrap());
        assert_eq!(builder.0.lock().unwrap().clone().wrappers.unwrap().len(), 1);
        assert_eq!(builder.get_wrappers().unwrap().len(), 1);
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

        let redirects = builder.0.lock().unwrap().clone().redirects.unwrap();
        assert_eq!(
            redirects,
            HashMap::from([
                (uri!("wrap/a"), uri!("wrap/b")),
                (uri!("wrap/c"), uri!("wrap/d")),
            ])
        );
        assert_eq!(builder.get_redirects().unwrap(), HashMap::from([
            ("wrap://wrap/a".to_string(), Arc::new(FFIUri(uri!("wrap/b")))),
            ("wrap://wrap/c".to_string(), Arc::new(FFIUri(uri!("wrap/d")))),
        ]));

        builder.remove_redirect(
            ffi_uri_from_string("wrap/a").unwrap(),
        );

        let redirects = builder.0.lock().unwrap().clone().redirects.unwrap();
        assert_eq!(
            redirects,
            HashMap::from([
                (uri!("wrap/c"), uri!("wrap/d")),
            ])
        );
        assert_eq!(builder.get_redirects().unwrap(), HashMap::from([
            ("wrap://wrap/c".to_string(), Arc::new(FFIUri(uri!("wrap/d")))),
        ]));
    }

    #[test]
    fn adds_and_removes_interface_implementation() {
        let builder = FFIBuilderConfig::new();
        let interface_uri = ffi_uri_from_string("wrap://mock/interface").unwrap();
        let implementation_a_uri = ffi_uri_from_string("wrap://mock/implementation-a").unwrap();
        let implementation_b_uri = ffi_uri_from_string("wrap://mock/implementation-b").unwrap();

        let implementation_c_uri = ffi_uri_from_string("wrap://mock/implementation-c").unwrap();
        builder.add_interface_implementations(
            interface_uri.clone(),
            vec![implementation_a_uri, implementation_b_uri.clone()],
        );
        builder.add_interface_implementation(interface_uri.clone(), implementation_c_uri.clone());

        let interfaces = builder
            .0
            .lock()
            .unwrap()
            .clone()
            .interfaces
            .unwrap();
        let implementations = interfaces.get(&interface_uri.0);
        assert_eq!(
            implementations,
            Some(&vec![
                uri!("wrap://mock/implementation-a"),
                uri!("wrap://mock/implementation-b"),
                uri!("wrap://mock/implementation-c")
            ])
        );
        assert_eq!(
            builder.get_interfaces(),
            Some(HashMap::from([
                (
                    "wrap://mock/interface".to_string(),
                    vec![
                        Arc::new(FFIUri(uri!("wrap://mock/implementation-a"))),
                        Arc::new(FFIUri(uri!("wrap://mock/implementation-b"))),
                        Arc::new(FFIUri(uri!("wrap://mock/implementation-c")))
                    ]
                )
            ]))
        );

        builder.remove_interface_implementation(interface_uri.clone(), implementation_b_uri);
        let interfaces = builder
            .0
            .lock()
            .unwrap()
            .clone()
            .interfaces
            .unwrap();
        let implementations = interfaces.get(&interface_uri.0);
        assert_eq!(
            implementations,
            Some(&vec![
                uri!("wrap://mock/implementation-a"),
                uri!("wrap://mock/implementation-c")
            ])
        );
        assert_eq!(
            builder.get_interfaces(),
            Some(HashMap::from([
                (
                    "wrap://mock/interface".to_string(),
                    vec![
                        Arc::new(FFIUri(uri!("wrap://mock/implementation-a"))),
                        Arc::new(FFIUri(uri!("wrap://mock/implementation-c")))
                    ]
                )
            ]))
        );
    }

    #[test]
    fn adds_and_get_resolvers() {
        let builder = FFIBuilderConfig::new();
        let static_resolver = FFIStaticUriResolver::new(HashMap::new()).unwrap();
        builder.add_resolver(Arc::new(FFIUriResolver::new(Box::new(static_resolver))));

        assert!(builder.0.lock().unwrap().resolvers.is_some());

        let resolvers = builder.get_resolvers().unwrap();
        assert_eq!(resolvers.len(), 1);
    }

    #[test]
    fn add_system_defaults() {
        let builder = FFIBuilderConfig::new();
        builder.add_system_defaults();
        assert!(builder.0.lock().unwrap().envs.is_some());
        assert!(builder.0.lock().unwrap().redirects.is_some());
        assert!(builder.0.lock().unwrap().interfaces.is_some());
        assert!(builder.0.lock().unwrap().wrappers.is_none());
        assert!(builder.0.lock().unwrap().packages.is_some());
    
        let _ = builder.build();
    }

    #[test]
    fn add_web3_defaults() {
        let builder = FFIBuilderConfig::new();
        builder.add_web3_defaults();
        assert!(builder.0.lock().unwrap().redirects.is_some());
        assert!(builder.0.lock().unwrap().interfaces.is_some());
        assert!(builder.0.lock().unwrap().packages.is_some());

        let _ = builder.build();
    }
}
