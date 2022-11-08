use polywrap_core::{
    uri_resolver::UriResolver,
    wrapper::Wrapper,
    package::WrapPackage
};

pub enum UriResolverLike {
    Wrapper(UriWrapper),
    Package(UriPackage),
    UriResolver(UriResolver),
    UriResolverLike(Vec<Self>)
}