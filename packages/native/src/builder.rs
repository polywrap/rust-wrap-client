use polywrap_client::{
    builder::types::{BuilderConfig as InnerBuilderConfig, ClientBuilder},
    core::{
        package::WrapPackage,
        resolvers::{uri_resolver::UriResolver, uri_resolver_like::UriResolverLike},
        wrapper::Wrapper,
    },
};
use std::sync::{Arc, Mutex};

pub struct BuilderConfigContainer {
    pub inner_builder: Mutex<InnerBuilderConfig>,
}

impl BuilderConfigContainer {
    pub fn new() -> BuilderConfigContainer {
        BuilderConfigContainer {
            inner_builder: Mutex::new(InnerBuilderConfig::new(None)),
        }
    }
}

pub fn new_builder_config() -> BuilderConfigContainer {
    BuilderConfigContainer::new()
}

pub fn add_env(builder: Arc<BuilderConfigContainer>, uri: &str, env: &str) {
    builder.inner_builder.lock().unwrap().add_env(
        uri.to_string().try_into().unwrap(),
        serde_json::from_str(env).unwrap(),
    );
}

pub fn remove_env(builder: Arc<BuilderConfigContainer>, uri: &str) {
    builder
        .inner_builder
        .lock()
        .unwrap()
        .remove_env(uri.to_string().try_into().unwrap());
}

pub fn set_env(builder: Arc<BuilderConfigContainer>, uri: &str, env: &str) {
    builder.inner_builder.lock().unwrap().set_env(
        uri.to_string().try_into().unwrap(),
        serde_json::from_str(env).unwrap(),
    );
}

pub fn add_interface_implementation(
    builder: Arc<BuilderConfigContainer>,
    interface_uri: &str,
    implementation_uri: &str,
) {
    builder
        .inner_builder
        .lock()
        .unwrap()
        .add_interface_implementation(
            interface_uri.try_into().unwrap(),
            implementation_uri.try_into().unwrap(),
        );
}

pub fn remove_interface_implementation(
    builder: Arc<BuilderConfigContainer>,
    interface_uri: &str,
    implementation_uri: &str,
) {
    builder
        .inner_builder
        .lock()
        .unwrap()
        .remove_interface_implementation(
            interface_uri.try_into().unwrap(),
            implementation_uri.try_into().unwrap(),
        );
}

pub fn add_wrapper(builder: Arc<BuilderConfigContainer>, uri: &str, wrapper: Box<dyn Wrapper>) {
    builder.inner_builder.lock().unwrap().add_wrapper(
        uri.to_string().try_into().unwrap(),
        Arc::new(Mutex::new(wrapper)),
    );
}

pub fn remove_wrapper(builder: Arc<BuilderConfigContainer>, uri: &str) {
    builder
        .inner_builder
        .lock()
        .unwrap()
        .remove_wrapper(uri.try_into().unwrap());
}

pub fn add_package(builder: Arc<BuilderConfigContainer>, uri: &str, package: Box<dyn WrapPackage>) {
    builder
        .inner_builder
        .lock()
        .unwrap()
        .add_package(uri.try_into().unwrap(), Arc::new(Mutex::new(package)));
}

pub fn remove_package(builder: Arc<BuilderConfigContainer>, uri: &str) {
    builder
        .inner_builder
        .lock()
        .unwrap()
        .remove_package(uri.try_into().unwrap());
}

pub fn add_redirect(builder: Arc<BuilderConfigContainer>, from: &str, to: &str) {
    builder.inner_builder.lock().unwrap().add_redirect(
        from.to_string().try_into().unwrap(),
        to.to_string().try_into().unwrap(),
    );
}

pub fn remove_redirect(builder: Arc<BuilderConfigContainer>, from: &str) {
    builder
        .inner_builder
        .lock()
        .unwrap()
        .remove_redirect(from.to_string().try_into().unwrap());
}

pub fn add_resolver(builder: Arc<BuilderConfigContainer>, resolver: Box<dyn UriResolver>) {
    builder
        .inner_builder
        .lock()
        .unwrap()
        .add_resolver(UriResolverLike::Resolver(Arc::from(resolver)));
}
