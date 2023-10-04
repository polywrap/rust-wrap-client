use std::{collections::HashMap, sync::Arc};

use polywrap_core::{
    client::CoreClientConfigBuilder, package::WrapPackage, resolution::uri_resolver::UriResolver,
    uri::Uri, wrapper::Wrapper,
};

use crate::ClientConfig;

/// Defines a type that uses the builder pattern to build a `ClientConfig`.
pub trait ClientConfigBuilder: CoreClientConfigBuilder {
    /// Merges a `ClientConfig` with the current instance's state.
    ///
    /// # Arguments
    ///
    /// * `config` - A `ClientConfig` instance to be merged with the current state.
    fn add(&mut self, config: ClientConfig) -> &mut Self;
    
    /// Adds an environment configuration entry.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` associated with the environment configuration.
    /// * `env` - A msgpack buffer representing the environment configuration.
    fn add_env(&mut self, uri: Uri, env: Vec<u8>) -> &mut Self;

    /// Adds several environment configuration entries.
    ///
    /// # Arguments
    ///
    /// * `env` - A HashMap where the key is the `Uri` and the value is a msgpack buffer representing the environment configuration.
    fn add_envs(&mut self, env: HashMap<Uri, Vec<u8>>) -> &mut Self;

    /// Removes an environment entry by `Uri`.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` of the environment entry to be removed.
    fn remove_env(&mut self, uri: &Uri) -> &mut Self;

    /// Adds an interface implementation entry.
    ///
    /// # Arguments
    ///
    /// * `interface_uri` - The `Uri` of the interface.
    /// * `implementation_uri` - The `Uri` of the implementation.
    fn add_interface_implementation(
        &mut self,
        interface_uri: Uri,
        implementation_uri: Uri,
    ) -> &mut Self;

    /// Adds several interface implementations.
    ///
    /// # Arguments
    ///
    /// * `interface_uri` - The `Uri` of the interface.
    /// * `implementation_uris` - A list of implementation `Uri`s.
    fn add_interface_implementations(
        &mut self,
        interface_uri: Uri,
        implementation_uris: Vec<Uri>,
    ) -> &mut Self;

    /// Removes an implementation from an interface.
    ///
    /// # Arguments
    ///
    /// * `interface_uri` - The `Uri` of the interface.
    /// * `implementation_uri` - The `Uri` of the implementation to be removed.
    fn remove_interface_implementation(
        &mut self,
        interface_uri: &Uri,
        implementation_uri: &Uri,
    ) -> &mut Self;

    /// Embeds a wrapper to the config.
    /// When invoking this wrapper's `Uri`, it won't be fetched at runtime; but taken from the config instead.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` of the `Wrapper`.
    /// * `wrapper` - A `Wrapper` instance.
    fn add_wrapper(&mut self, uri: Uri, wrapper: Arc<dyn Wrapper>) -> &mut Self;

    /// Embeds several wrappers to the config.
    ///
    /// # Arguments
    ///
    /// * `wrappers` - A list of tuples where each tuple contains a `Uri` and a `Wrapper` instance.
    fn add_wrappers(&mut self, wrappers: Vec<(Uri, Arc<dyn Wrapper>)>) -> &mut Self;

    /// Removes an embedded wrapper.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` of the wrapper to be removed.
    fn remove_wrapper(&mut self, uri: &Uri) -> &mut Self;

    /// Embeds a `WrapPackage` to the config.
    /// When invoking this package's `Uri`, the embedded local instance will create a `Wrapper` and invoke it.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` of the `WrapPackage`.
    /// * `package` - A `WrapPackage` instance.
    fn add_package(&mut self, uri: Uri, package: Arc<dyn WrapPackage>) -> &mut Self;

    /// Embeds several `WrapPackage`s to the config.
    ///
    /// # Arguments
    ///
    /// * `packages` - A list of tuples where each tuple contains a `Uri` and a `WrapPackage` instance.
    fn add_packages(&mut self, packages: Vec<(Uri, Arc<dyn WrapPackage>)>) -> &mut Self;

    /// Removes an embedded `WrapPackage`.
    ///
    /// # Arguments
    ///
    /// * `uri` - The `Uri` of the `WrapPackage` to be removed.
    fn remove_package(&mut self, uri: &Uri) -> &mut Self;

    /// Specifies a `Uri` that should be redirected to another `Uri`.
    ///
    /// # Arguments
    ///
    /// * `from` - The original `Uri`.
    /// * `to` - The `Uri` to which the original `Uri` should be redirected.
    fn add_redirect(&mut self, from: Uri, to: Uri) -> &mut Self;

    /// Specifies multiple `Uri`s that should be redirected to other `Uri`s.
    ///
    /// # Arguments
    ///
    /// * `redirects` - A HashMap where the key is the original `Uri` and the value is the `Uri` to which it should be redirected.
    fn add_redirects(&mut self, redirects: HashMap<Uri, Uri>) -> &mut Self;

    /// Removes a previously added redirect from one `Uri` to another.
    ///
    /// # Arguments
    ///
    /// * `from` - The original `Uri` of the redirect to be removed.
    fn remove_redirect(&mut self, from: &Uri) -> &mut Self;
    /// Adds a custom `Uri` resolver to the configuration.
    ///
    /// # Arguments
    ///
    /// * `resolver` - A UriResolver instance.
    fn add_resolver(&mut self, resolver: Arc<dyn UriResolver>) -> &mut Self;

    /// Adds multiple custom `Uri` resolvers to the configuration.
    ///
    /// # Arguments
    ///
    /// * `resolvers` - A list of UriResolver instances.
    fn add_resolvers(&mut self, resolvers: Vec<Arc<dyn UriResolver>>) -> &mut Self;
}
