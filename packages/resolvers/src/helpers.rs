use polywrap_core::{
    uri_resolver::UriResolver,
    uri_resolution_context::{UriWrapper,UriPackage},
    package::WrapPackage
};

pub enum UriResolverLike {
    Wrapper(UriWrapper),
    Package(UriPackage),
    UriResolver(Box<dyn UriResolver>),
    UriResolverLike(Vec<Self>)
}