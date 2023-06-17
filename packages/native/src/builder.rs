use std::sync::{Arc, Mutex};

use polywrap_client::{builder::{PolywrapClientConfig, PolywrapClientConfigBuilder}, client::PolywrapClient};

use crate::{
    client::FFIClient,
    package::{FFIWrapPackage, WrapPackageWrapping},
    resolvers::ffi_resolver::{FFIUriResolver, UriResolverWrapping},
    uri::FFIUri,
    wrapper::{FFIWrapper, WrapperWrapping},
};

pub struct FFIBuilderConfig {
    pub inner_builder: Mutex<PolywrapClientConfig>,
}

impl FFIBuilderConfig {
    pub fn new() -> FFIBuilderConfig {
        FFIBuilderConfig {
            inner_builder: Mutex::new(PolywrapClientConfig::new()),
        }
    }

    pub fn add_env(&self, uri: Arc<FFIUri>, env: Vec<u8>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_env(uri.0.clone(), env);
    }

    pub fn remove_env(&self, uri: Arc<FFIUri>) {
        self.inner_builder.lock().unwrap().remove_env(&uri.0);
    }

    pub fn add_interface_implementation(
        &self,
        interface_uri: Arc<FFIUri>,
        implementation_uri: Arc<FFIUri>,
    ) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_interface_implementation(interface_uri.0.clone(), implementation_uri.0.clone());
    }

    pub fn remove_interface_implementation(
        &self,
        interface_uri: Arc<FFIUri>,
        implementation_uri: Arc<FFIUri>,
    ) {
        self.inner_builder
            .lock()
            .unwrap()
            .remove_interface_implementation(&interface_uri.0, &implementation_uri.0);
    }

    pub fn add_wrapper(&self, uri: Arc<FFIUri>, wrapper: Box<dyn FFIWrapper>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_wrapper(uri.0.clone(), Arc::new(WrapperWrapping(wrapper)));
    }

    pub fn remove_wrapper(&self, uri: Arc<FFIUri>) {
        self.inner_builder.lock().unwrap().remove_wrapper(&uri.0);
    }

    pub fn add_package(&self, uri: Arc<FFIUri>, package: Box<dyn FFIWrapPackage>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_package(uri.0.clone(), Arc::new(WrapPackageWrapping(package)));
    }

    pub fn remove_package(&self, uri: Arc<FFIUri>) {
        self.inner_builder.lock().unwrap().remove_package(&uri.0);
    }

    pub fn add_redirect(&self, from: Arc<FFIUri>, to: Arc<FFIUri>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_redirect(from.0.clone(), to.0.clone());
    }

    pub fn remove_redirect(&self, from: Arc<FFIUri>) {
        self.inner_builder.lock().unwrap().remove_redirect(&from.0);
    }

    pub fn add_resolver(&self, resolver: Box<dyn FFIUriResolver>) {
        self.inner_builder
            .lock()
            .unwrap()
            .add_resolver(Arc::from(UriResolverWrapping(resolver).as_uri_resolver()));
    }

    pub fn build(&self) -> Arc<FFIClient> {
        let config = self.inner_builder.lock().unwrap().clone();
        let client = Arc::new(PolywrapClient::new(config.into()));
        Arc::new(FFIClient::new(client))
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use polywrap_client::{
        core::{client::UriRedirect, uri::Uri},
        msgpack::msgpack,
    };
    use polywrap_tests_utils::mocks::{
        get_different_mock_package, get_different_mock_wrapper, get_mock_package, get_mock_wrapper,
    };

    use crate::{package::FFIWrapPackage, uri::FFIUri, wrapper::FFIWrapper};

    use super::FFIBuilderConfig;

    #[test]
    fn adds_and_removes_env() {
        let builder = FFIBuilderConfig::new();
        let uri = Arc::new(FFIUri::from_string("wrap://ens/some.eth"));
        let env = msgpack!({
          "foo": "bar"
        });

        builder.add_env(uri.clone(), env.clone());

        let envs = builder.inner_builder.lock().unwrap().clone().envs.unwrap();
        let current_env = envs.get("wrap://ens/some.eth");
        assert_eq!(&env, current_env.unwrap());

        let new_env = msgpack!({
            "bar": "foo"
        });
        builder.add_env(uri.clone(), new_env.clone());
        let envs = builder.inner_builder.lock().unwrap().clone().envs.unwrap();
        let current_env = envs.get("wrap://ens/some.eth");
        assert_eq!(&new_env, current_env.unwrap());

        builder.remove_env(uri);
        let envs = builder.inner_builder.lock().unwrap().clone().envs;
        assert_eq!(None, envs);
    }

    #[test]
    fn adds_and_removes_package() {
        let builder = FFIBuilderConfig::new();
        let mock_package: Box<dyn FFIWrapPackage> = Box::new(get_mock_package());
        let uri_mock_package = Arc::new(FFIUri::from_string("package/a"));
        let different_mock_package: Box<dyn FFIWrapPackage> =
            Box::new(get_different_mock_package());
        let uri_different_mock_package = Arc::new(FFIUri::from_string("package/b"));

        builder.add_package(uri_mock_package.clone(), mock_package);
        builder.add_package(uri_different_mock_package, different_mock_package);
        assert_eq!(
            builder
                .inner_builder
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
                .inner_builder
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
        let uri_mock_wrapper = Arc::new(FFIUri::from_string("wrap/a"));
        let different_mock_wrapper: Box<dyn FFIWrapper> = Box::new(get_different_mock_wrapper());
        let uri_different_mock_wrapper = Arc::new(FFIUri::from_string("wrap/b"));

        builder.add_wrapper(uri_mock_wrapper.clone(), mock_package);
        builder.add_wrapper(uri_different_mock_wrapper, different_mock_wrapper);
        assert_eq!(
            builder
                .inner_builder
                .lock()
                .unwrap()
                .clone()
                .wrappers
                .unwrap()
                .len(),
            2
        );

        builder.remove_wrapper(uri_mock_wrapper);
        assert_eq!(
            builder
                .inner_builder
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
            Arc::new(FFIUri::from_string("wrap/a")),
            Arc::new(FFIUri::from_string("wrap/b")),
        );
        builder.add_redirect(
            Arc::new(FFIUri::from_string("wrap/c")),
            Arc::new(FFIUri::from_string("wrap/d")),
        );

        let redirects = builder
            .inner_builder
            .lock()
            .unwrap()
            .clone()
            .redirects
            .unwrap();
        assert_eq!(
            redirects,
            vec![
                UriRedirect {
                    from: "wrap/a".to_string().try_into().unwrap(),
                    to: "wrap/b".to_string().try_into().unwrap()
                },
                UriRedirect {
                    from: "wrap/c".to_string().try_into().unwrap(),
                    to: "wrap/d".to_string().try_into().unwrap()
                }
            ]
        );
    }

    #[test]
    fn adds_and_removes_interface_implementation() {
        let builder = FFIBuilderConfig::new();
        let interface_uri = Arc::new(FFIUri::from_string("wrap://ens/interface.eth"));
        let implementation_a_uri = Arc::new(FFIUri::from_string("wrap://ens/implementation-a.eth"));
        let implementation_b_uri = Arc::new(FFIUri::from_string("wrap://ens/implementation-b.eth"));

        builder.add_interface_implementation(interface_uri.clone(), implementation_a_uri);
        builder.add_interface_implementation(interface_uri.clone(), implementation_b_uri.clone());

        let interfaces: std::collections::HashMap<String, Vec<polywrap_client::core::uri::Uri>> =
            builder
                .inner_builder
                .lock()
                .unwrap()
                .clone()
                .interfaces
                .unwrap();
        let implementations = interfaces.get(&interface_uri.to_string());
        assert_eq!(
            implementations,
            Some(&vec![
                Uri::new("wrap://ens/implementation-a.eth"),
                Uri::new("wrap://ens/implementation-b.eth")
            ])
        );

        builder.remove_interface_implementation(interface_uri.clone(), implementation_b_uri);
        let interfaces: std::collections::HashMap<String, Vec<polywrap_client::core::uri::Uri>> =
            builder
                .inner_builder
                .lock()
                .unwrap()
                .clone()
                .interfaces
                .unwrap();
        let implementations = interfaces.get(&interface_uri.to_string());
        assert_eq!(
            implementations,
            Some(&vec![Uri::new("wrap://ens/implementation-a.eth"),])
        );
    }
}
