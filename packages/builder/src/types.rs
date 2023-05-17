use std::sync::{Arc};

use polywrap_core::{
    interface_implementation::InterfaceImplementations,
    env::{Envs,Env}, 
    uri::Uri, 
    client::{UriRedirect, ClientConfig}, package::WrapPackage, wrapper::Wrapper, resolvers::uri_resolver::UriResolver
};

#[derive(Clone)]
pub struct BuilderConfig {
    pub interfaces: Option<InterfaceImplementations>,
    pub envs: Option<Envs>,
    pub wrappers: Option<Vec<(Uri, Arc<dyn Wrapper>)>>,
    pub packages: Option<Vec<(Uri, Arc<dyn WrapPackage>)>>,
    pub redirects: Option<Vec<UriRedirect>>,
    pub resolvers: Option<Vec<Arc<dyn UriResolver>>>,
}

pub trait ClientBuilder {
    fn add(&mut self, config: BuilderConfig) -> &mut Self;
    fn add_env(&mut self, uri: Uri, env: Env) -> &mut Self;
    fn add_envs(&mut self, env: Envs) -> &mut Self;
    fn remove_env(&mut self, uri: &Uri) -> &mut Self;
    fn set_env(&mut self, uri: Uri, env: Env) -> &mut Self;
    fn add_interface_implementation(
        &mut self, 
        interface_uri: Uri,
        implementation_uri: Uri
    ) -> &mut Self;
    fn add_interface_implementations(
        &mut self, 
        interface_uri: Uri,
        implementation_uris: Vec<Uri>
    ) -> &mut Self;
    fn remove_interface_implementation(
        &mut self,
        interface_uri: &Uri,
        implementation_uri: &Uri
    ) -> &mut Self;
    fn add_wrapper(&mut self, uri: Uri, wrapper: Arc<dyn Wrapper>) -> &mut Self;
    fn add_wrappers(&mut self, wrappers: Vec<(Uri, Arc<dyn Wrapper>)>) -> &mut Self;
    fn remove_wrapper(&mut self, uri: &Uri) -> &mut Self;
    fn add_package(&mut self, uri: Uri, package: Arc<dyn WrapPackage>) -> &mut Self;
    fn add_packages(&mut self, packages: Vec<(Uri, Arc<dyn WrapPackage>)>) -> &mut Self;
    fn remove_package(&mut self, uri: &Uri) -> &mut Self;
    fn add_redirect(&mut self, from: Uri, to: Uri) -> &mut Self;
    fn add_redirects(&mut self, redirects: Vec<UriRedirect>) -> &mut Self;
    fn remove_redirect(&mut self, from: &Uri) -> &mut Self;
    fn add_resolver(&mut self, resolver: Arc<dyn UriResolver>) -> &mut Self;
    fn add_resolvers(&mut self, resolver: Vec<Arc<dyn UriResolver>>) -> &mut Self;
}

pub trait ClientConfigHandler {
    fn build(self) -> ClientConfig;
}

