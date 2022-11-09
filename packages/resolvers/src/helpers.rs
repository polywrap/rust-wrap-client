use polywrap_core::{
    uri::Uri,
    uri_resolution_context::{UriWrapper,UriPackage},
};

pub enum UriResolverLike {
    Wrapper(UriWrapper),
    Package(UriPackage),
    UriResolver(Uri),
    UriResolverLike(Vec<Self>)
}