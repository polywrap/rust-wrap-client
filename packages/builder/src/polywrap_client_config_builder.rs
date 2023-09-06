use std::{collections::HashMap, sync::Arc};

use polywrap_core::{
    client::CoreClientConfigBuilder, package::WrapPackage, resolution::uri_resolver::UriResolver,
    uri::Uri, wrapper::Wrapper,
};

use crate::ClientConfig;

pub trait ClientConfigBuilder: CoreClientConfigBuilder {
    fn add(&mut self, config: ClientConfig) -> &mut Self;
    fn add_env(&mut self, uri: Uri, env: Vec<u8>) -> &mut Self;
    fn add_envs(&mut self, env: HashMap<Uri, Vec<u8>>) -> &mut Self;
    fn remove_env(&mut self, uri: &Uri) -> &mut Self;
    fn add_interface_implementation(
        &mut self,
        interface_uri: Uri,
        implementation_uri: Uri,
    ) -> &mut Self;
    fn add_interface_implementations(
        &mut self,
        interface_uri: Uri,
        implementation_uris: Vec<Uri>,
    ) -> &mut Self;
    fn remove_interface_implementation(
        &mut self,
        interface_uri: &Uri,
        implementation_uri: &Uri,
    ) -> &mut Self;
    fn add_wrapper(&mut self, uri: Uri, wrapper: Arc<dyn Wrapper>) -> &mut Self;
    fn add_wrappers(&mut self, wrappers: Vec<(Uri, Arc<dyn Wrapper>)>) -> &mut Self;
    fn remove_wrapper(&mut self, uri: &Uri) -> &mut Self;
    fn add_packages(&mut self, packages: Vec<(Uri, Arc<dyn WrapPackage>)>) -> &mut Self;
    fn add_package(&mut self, uri: Uri, package: Arc<dyn WrapPackage>) -> &mut Self;
    fn remove_package(&mut self, uri: &Uri) -> &mut Self;
    fn add_redirect(&mut self, from: Uri, to: Uri) -> &mut Self;
    fn add_redirects(&mut self, redirects: HashMap<Uri, Uri>) -> &mut Self;
    fn remove_redirect(&mut self, from: &Uri) -> &mut Self;
    fn add_resolver(&mut self, resolver: Arc<dyn UriResolver>) -> &mut Self;
    fn add_resolvers(&mut self, resolver: Vec<Arc<dyn UriResolver>>) -> &mut Self;
}
