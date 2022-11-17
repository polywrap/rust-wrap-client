use std::collections::HashMap;

use polywrap_core::{
    client::{ClientConfig, UriRedirect},
    env::{Env,Envs},
    interface_implementation::InterfaceImplementations, 
    uri_resolution_context::{UriWrapper, UriPackage}, 
    uri::Uri
};
use polywrap_resolvers::static_::static_resolver::UriResolverLike;

pub struct BuilderConfig {
    client_config: Option<ClientConfig>,
    wrappers: Vec<UriWrapper>,
    packages: Vec<UriPackage>,
    redirects: Vec<UriRedirect>,
    resolvers: Vec<UriResolverLike>,
}

impl BuilderConfig {
    pub fn new(config: BuilderConfig) -> Self {
        BuilderConfig { 
            client_config: None, 
            wrappers: vec![], 
            packages: vec![], 
            redirects: vec![], 
            resolvers: vec![] 
        }
    }

    pub fn add(&mut self, config: BuilderConfig) -> &mut Self {
        self
    }

    pub fn add_wrapper(&mut self, wrapper: UriWrapper) -> &mut Self {
        self.wrappers.push(wrapper);
        self
    }

    pub fn remove_wrapper(&mut self, uri: Uri) -> &mut Self {
        if let Some(index) = self.wrappers.iter().position(|wrapper| wrapper.uri == uri) {
            self.wrappers.remove(index);
        }
        self
    }

    pub fn add_package(&mut self, package: UriPackage) -> &mut Self {
        let existing_package = self.packages
            .iter_mut()
            .find(|i| i.uri == package.uri);

        if let Some(p) = existing_package {
            p.package = package.package;
        } else {
            self.packages.push(package);
        }
        self
    }

    pub fn add_packages(&mut self, packages: Vec<UriPackage>) -> &mut Self {
        for package in packages.into_iter() {
            self.add_package(package);
        }
        self
    }

    pub fn remove_package(&mut self, uri: Uri) -> &mut Self {
        if let Some(index) = self.packages.iter().position(|package| package.uri == uri) {
            self.packages.remove(index);
        }
        self
    }

    pub fn add_env(&mut self, uri: Uri, env: Env) -> &mut Self {
        if let Some(c) = self.client_config.as_mut() {
             if let Some(envs) = c.envs.as_mut() {
                envs.insert(uri.uri, env);
            } else {
                let mut envs: Envs = HashMap::new();
                envs.insert(uri.uri, env);
                c.envs = Some(envs);
            }
        }
        self
    }

    pub fn add_envs(&mut self, envs: Envs) -> &mut Self {
        for (uri, env) in envs.into_iter() {
            self.add_env(Uri::new(uri.as_str()), env);
        }
        self
    }

    pub fn remove_env(&mut self, uri: Uri) -> &mut Self {
        if let Some(c) = self.client_config.as_mut() {
            if let Some(envs) = c.envs.as_mut() {
                envs.retain(|k, _| k != &uri.uri)
            }
        }
        self
    }

    pub fn set_env(&mut self, env: Env) -> &mut Self {
        self
    }

    pub fn add_interface_implementation(
        &mut self, 
        interface_uri: Uri,
        implementation_uri: Uri
    ) -> &mut Self {
        self
    }

    pub fn add_interface_implementations(
        &mut self, 
        interface_uri: Uri,
        implementation_uris: Vec<Uri>
    ) -> &mut Self {
        self
    }

    pub fn remove_interface_implementation(
        &mut self,
        implementations: InterfaceImplementations
    ) -> &mut Self {
        self
    }

    pub fn add_redirect(&mut self, from: Uri, to: Uri) -> &mut Self {
        self
    }

    pub fn remove_redirect(&mut self, from: Uri) -> &mut Self {
        self
    }

    pub fn add_resolver(&mut self, resolver: UriResolverLike) -> &mut Self {
        self
    }

    pub fn add_resolvers(&mut self, resolver: Vec<UriResolverLike>) -> &mut Self {
        self
    }

    pub fn build(&self) -> &Self {
        self
    }

    // pub fn build_core(&self) -> ClientConfig {
    //     self.client_config
    // }
}