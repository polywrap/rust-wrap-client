use polywrap_core::{
    uri_resolver::UriResolver,
    wrapper::Wrapper,
    package::WrapPackage
};

pub enum UriResolverLike {
    Wrapper(Wrapper),
    Package(WrapPackage),
    UriResolver(UriResolver),
    UriResolverLike(Vec<Self>)
}