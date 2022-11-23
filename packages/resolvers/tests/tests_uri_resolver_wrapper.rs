use async_trait::async_trait;
use polywrap_core::uri::Uri;
use polywrap_resolvers::{uri_resolver_wrapper::UriResolverWrapper, resolver_with_history::ResolverWithHistory};
use polywrap_tests_utils::helpers::get_tests_path;

#[tokio::test]
async fn uri_resolver_wrapper_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();
    let wrapper_path = format!("{}/subinvoke/00-subinvoke/implementations/as", path);
    let wrapper_uri = Uri::try_from(format!("fs/{}", wrapper_path)).unwrap();
    let uri_resolver_wrapper = UriResolverWrapper::new(wrapper_uri);

    
}