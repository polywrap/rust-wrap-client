use std::{collections::HashMap, sync::Arc};

use polywrap_core::{
    client::{ClientConfig, UriRedirect, ClientConfigBuilder},
    interface_implementation::InterfaceImplementations,
    package::WrapPackage,
    resolution::uri_resolver::UriResolver,
    uri::Uri,
    wrapper::Wrapper,
};

use crate::{
    build_static_resolver, PolywrapBaseResolver, PolywrapBaseResolverOptions,
    PolywrapClientConfigBuilder,
};

#[derive(Default, Clone)]
pub struct PolywrapClientConfig {
    pub interfaces: Option<InterfaceImplementations>,
    pub envs: Option<HashMap<String, Vec<u8>>>,
    pub wrappers: Option<Vec<(Uri, Arc<dyn Wrapper>)>>,
    pub packages: Option<Vec<(Uri, Arc<dyn WrapPackage>)>>,
    pub redirects: Option<Vec<UriRedirect>>,
    pub resolvers: Option<Vec<Arc<dyn UriResolver>>>,
}

impl PolywrapClientConfig {
    pub fn new() -> Self {
        // We don't want to use the default constructor here because it may change
        // and then `new` would no longer create an empty config.
        Self {
            interfaces: None,
            envs: None,
            wrappers: None,
            packages: None,
            redirects: None,
            resolvers: None,
        }
    }
}

impl PolywrapClientConfigBuilder for PolywrapClientConfig {
    fn add(&mut self, config: PolywrapClientConfig) -> &mut Self {
        if let Some(e) = config.envs {
            self.add_envs(e);
        };

        if let Some(i) = config.interfaces {
            for (interface, implementation_uris) in i.into_iter() {
                let interface_uri: Uri = interface.try_into().unwrap();
                self.add_interface_implementations(interface_uri, implementation_uris);
            }
        };

        if let Some(r) = config.redirects {
            self.add_redirects(r);
        }

        if let Some(w) = config.wrappers {
            self.add_wrappers(w);
        }

        if let Some(p) = config.packages {
            self.add_packages(p);
        }

        if let Some(resolvers) = config.resolvers {
            self.add_resolvers(resolvers);
        }

        self
    }

    fn add_env(&mut self, uri: Uri, env: Vec<u8>) -> &mut Self {
        match self.envs.as_mut() {
            Some(envs) => {
                envs.insert(uri.to_string(), env);
            }
            None => {
                let mut envs: HashMap<String, Vec<u8>> = HashMap::new();
                envs.insert(uri.to_string(), env);
                self.envs = Some(envs);
            }
        };
        self
    }

    fn add_envs(&mut self, envs: HashMap<String, Vec<u8>>) -> &mut Self {
        for (uri, env) in envs.into_iter() {
            self.add_env(Uri::new(uri.as_str()), env);
        }
        self
    }

    fn remove_env(&mut self, uri: &Uri) -> &mut Self {
        if let Some(envs) = self.envs.as_mut() {
            envs.retain(|k, _| &uri.clone().uri != k);
            if envs.keys().len() == 0 {
                self.envs = None;
            }
        }
        self
    }

    fn add_interface_implementation(
        &mut self,
        interface_uri: Uri,
        implementation_uri: Uri,
    ) -> &mut Self {
        match self.interfaces.as_mut() {
            Some(interfaces) => {
                let current_interface = interfaces.get_mut(&interface_uri.to_string());
                match current_interface {
                    Some(i) => i.push(implementation_uri),
                    None => {
                        interfaces.insert(interface_uri.to_string(), vec![implementation_uri]);
                    }
                }
            }
            None => {
                let mut interfaces = HashMap::new();
                interfaces.insert(interface_uri.to_string(), vec![implementation_uri]);
                self.interfaces = Some(interfaces);
            }
        }
        self
    }

    fn add_interface_implementations(
        &mut self,
        interface_uri: Uri,
        implementation_uris: Vec<Uri>,
    ) -> &mut Self {
        match self.interfaces.as_mut() {
            Some(interfaces) => {
                let current_interface = interfaces.get_mut(&interface_uri.to_string());
                match current_interface {
                    Some(i) => {
                        for implementation_uri in implementation_uris {
                            if !i.contains(&implementation_uri) {
                                i.push(implementation_uri);
                            }
                        }
                    }
                    None => {
                        interfaces.insert(interface_uri.to_string(), implementation_uris);
                    }
                };
            }
            None => {
                let mut interfaces = HashMap::new();
                interfaces.insert(interface_uri.to_string(), implementation_uris);
                self.interfaces = Some(interfaces);
            }
        };

        self
    }

    fn remove_interface_implementation(
        &mut self,
        interface_uri: &Uri,
        implementation_uri: &Uri,
    ) -> &mut Self {
        if let Some(interfaces) = self.interfaces.as_mut() {
            let implementations = interfaces.get_mut(&interface_uri.to_string());
            if let Some(implementations) = implementations {
                let index = implementations.iter().position(|i| i == implementation_uri);
                if let Some(i) = index {
                    implementations.remove(i);
                };
            };
        };

        self
    }

    fn add_wrapper(&mut self, uri: Uri, wrapper: Arc<dyn Wrapper>) -> &mut Self {
        if let Some(wrappers) = self.wrappers.as_mut() {
            let existing_wrapper = wrappers
                .iter_mut()
                .find(|i: &&mut (Uri, Arc<dyn Wrapper>)| i.0 == uri);

            if let Some(p) = existing_wrapper {
                p.1 = wrapper;
            } else {
                wrappers.push((uri, wrapper));
            }
        } else {
            self.wrappers = Some(vec![(uri, wrapper)]);
        }
        self
    }

    fn add_wrappers(&mut self, wrappers: Vec<(Uri, Arc<dyn Wrapper>)>) -> &mut Self {
        for (uri, wrapper) in wrappers.into_iter() {
            self.add_wrapper(uri, wrapper);
        }
        self
    }

    fn remove_wrapper(&mut self, uri: &Uri) -> &mut Self {
        if let Some(wrappers) = self.wrappers.as_mut() {
            if let Some(index) = wrappers
                .iter()
                .position(|(current_uri, _)| current_uri == uri)
            {
                wrappers.remove(index);
            }
        }
        self
    }

    fn add_package(&mut self, uri: Uri, package: Arc<dyn WrapPackage>) -> &mut Self {
        if let Some(packages) = self.packages.as_mut() {
            let existing_package = packages.iter_mut().find(|i| i.0 == uri);

            if let Some(p) = existing_package {
                p.1 = package;
            } else {
                packages.push((uri, package));
            }
        } else {
            self.packages = Some(vec![(uri, package)]);
        }
        self
    }

    fn add_packages(&mut self, packages: Vec<(Uri, Arc<dyn WrapPackage>)>) -> &mut Self {
        for (uri, package) in packages.into_iter() {
            self.add_package(uri, package);
        }
        self
    }

    fn remove_package(&mut self, uri: &Uri) -> &mut Self {
        if let Some(packages) = self.packages.as_mut() {
            if let Some(index) = packages
                .iter()
                .position(|(current_uri, _)| current_uri == uri)
            {
                packages.remove(index);
            }
        }
        self
    }

    fn add_redirect(&mut self, from: Uri, to: Uri) -> &mut Self {
        let redirect = UriRedirect {
            from: from.clone(),
            to,
        };
        match self.redirects.as_mut() {
            Some(redirects) => {
                if !redirects.iter().any(|u| u.from == from) {
                    redirects.push(redirect);
                }
            }
            None => {
                self.redirects = Some(vec![redirect]);
            }
        }

        self
    }

    fn add_redirects(&mut self, redirects: Vec<UriRedirect>) -> &mut Self {
        for UriRedirect { from, to } in redirects.into_iter() {
            self.add_redirect(from, to);
        }
        self
    }

    fn remove_redirect(&mut self, from: &Uri) -> &mut Self {
        if let Some(redirects) = self.redirects.as_mut() {
            if let Some(i) = redirects.iter().position(|u| &u.from == from) {
                redirects.remove(i);
                if redirects.is_empty() {
                    self.redirects = None;
                }
            };
        };

        self
    }

    fn add_resolver(&mut self, resolver: Arc<dyn UriResolver>) -> &mut Self {
        match self.resolvers.as_mut() {
            Some(resolvers) => {
                resolvers.push(resolver);
            }
            None => {
                self.resolvers = Some(vec![resolver]);
            }
        };

        self
    }

    fn add_resolvers(&mut self, resolvers: Vec<Arc<dyn UriResolver>>) -> &mut Self {
        for resolver in resolvers.into_iter() {
            self.add_resolver(resolver);
        }
        self
    }
}

impl ClientConfigBuilder for PolywrapClientConfig {
    fn build(self) -> ClientConfig {
        // We first build the resolver because it needs a reference to self
        // this way we don't need to clone `resolvers`, `envs`, and `interfaces`.
        ClientConfig {
            resolver: PolywrapBaseResolver::new(PolywrapBaseResolverOptions {
                static_resolver: build_static_resolver(&self),
                dynamic_resolvers: self.resolvers,
                ..Default::default()
            }),
            envs: self.envs,
            interfaces: self.interfaces,
        }
    }
}

impl Into<ClientConfig> for PolywrapClientConfig {
    fn into(self) -> ClientConfig {
        self.build()
    }
}
