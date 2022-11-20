use std::collections::HashMap;

use polywrap_core::{
    client::{ClientConfig, UriRedirect},
    env::{Env,Envs},
    interface_implementation::InterfaceImplementations, 
    uri_resolution_context::{UriWrapper, UriPackage}, 
    uri::Uri
};
use polywrap_resolvers::static_::static_resolver::UriResolverLike;

use crate::helpers::merge;

pub struct BuilderConfig {
    pub interfaces: Option<InterfaceImplementations>,
    pub envs: Option<Envs>,
    pub wrappers: Option<Vec<UriWrapper>>,
    pub packages: Option<Vec<UriPackage>>,
    pub redirects: Option<Vec<UriRedirect>>,
    pub resolvers: Option<Vec<UriResolverLike>>,
}

impl BuilderConfig {
    pub fn new(config: Option<BuilderConfig>) -> Self {
        if let Some(c) = config {
            let mut builder = BuilderConfig::new(None);
            builder.add(c);
        } 

        BuilderConfig { 
            interfaces: None,
            envs: None,
            wrappers: None,
            packages: None,
            redirects: None,
            resolvers: None
        }
    }

    pub fn add(&mut self, config: BuilderConfig) -> &mut Self {
        self
    }

    pub fn add_resolver(&mut self, resolver: UriResolverLike) -> &mut Self {
        self
    }

    pub fn add_env(&mut self, uri: Uri, env: Env) -> &mut Self {
        match self.envs.as_mut() {
            Some(envs) => {
                if let Some(u) = envs.get_mut(&uri.clone().uri) {
                    merge(u, &env.clone());
                } else {
                    envs.insert(uri.clone().uri, env.clone());
                }
            },
            None => {
                let mut envs: Envs = HashMap::new();
                envs.insert(uri.uri, env);
                self.envs = Some(envs);
            }
        };
        self
    }

    pub fn add_envs(&mut self, envs: Envs) -> &mut Self {
        for (uri, env) in envs.into_iter() {
            self.add_env(Uri::new(uri.as_str()), env);
        }
        self
    }

    pub fn remove_env(&mut self, uri: Uri) -> &mut Self {
        if let Some(envs) = self.envs.as_mut() {
            envs.retain(|env| env.keys().any(|k| k != &uri.clone().uri))
        }
        self
    }

    pub fn set_env(&mut self, uri: Uri, env: Env) -> &mut Self {
        match self.envs.as_mut() {
            Some(envs) => {
                let current_env = envs.keys().any(|k| k == &uri.clone().uri);
                if current_env  {
                    let mut new_env: Envs = HashMap::new();
                    new_env.insert(uri.clone().uri, env);
                    envs = new_env.clone();
                } else {
                    envs.insert(uri.clone().uri, env);
                }
            },
            None => {
                let mut new_env: Envs = HashMap::new();
                new_env.insert(uri.clone().uri, env);
                self.envs = Some(vec![new_env]);
            }
        }
        self
    }

    pub fn add_interface_implementation(
        &mut self, 
        interface_uri: Uri,
        implementation_uri: Uri
    ) -> &mut Self {
        match self.interfaces.as_mut() {
            Some(interfaces) => {
                let get_current_interface = |i: &&mut InterfaceImplementations| {
                    let has_interface_uri = |k| k == &interface_uri.clone().uri;
                    i.keys().any(has_interface_uri)
                };

                let current_interface = interfaces.iter_mut().find(get_current_interface);
                if let Some(c) = current_interface {
                    
                }
            },
            None => {

            }
        }

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

    pub fn add_wrapper(&mut self, wrapper: UriWrapper) -> &mut Self {
        if let Some(wrappers) = self.wrappers.as_mut() {
            wrappers.push(wrapper);
        } else {
            self.wrappers = Some(vec![wrapper]);
        }
        self
    }

    pub fn remove_wrapper(&mut self, uri: Uri) -> &mut Self {
        if let Some(wrappers) = self.wrappers.as_mut() {
            if let Some(index) = wrappers.iter().position(|wrapper| wrapper.uri == uri) {
                wrappers.remove(index);
            }
        }
        self
    }

    pub fn add_package(&mut self, package: UriPackage) -> &mut Self {
        if let Some(packages) = self.packages.as_mut() {
            let existing_package = packages
            .iter_mut()
            .find(|i| i.uri == package.uri);
            
            if let Some(p) = existing_package {
                p.package = package.package;
            } else {
                packages.push(package);
            } 
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
        if let Some(packages) = self.packages.as_mut() {
            if let Some(index) = packages.iter().position(|package| package.uri == uri) {
                packages.remove(index);
            }
        }
        self
    }


    pub fn add_redirect(&mut self, from: Uri, to: Uri) -> &mut Self {
        self
    }

    pub fn remove_redirect(&mut self, from: Uri) -> &mut Self {
        self
    }

    pub fn add_resolvers(&mut self, resolver: Vec<UriResolverLike>) -> &mut Self {
        self
    }

    pub fn build(&self) -> &ClientConfig {
        &ClientConfig {
            resolver: todo!(),
            envs: todo!(),
            interfaces: todo!(),
        }
    }
}