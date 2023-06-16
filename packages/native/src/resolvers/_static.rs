use polywrap_client::{
    core::resolution::{uri_resolution_context::UriPackageOrWrapper, uri_resolver::UriResolver},
    resolvers::static_resolver::StaticResolver,
};
use std::{collections::HashMap, sync::Arc};

use crate::{
    error::FFIError,
    invoker::{FFIInvoker, FFIInvokerWrapping},
    uri::FFIUri,
};

use super::{
    ffi_resolver::FFIUriResolver, resolution_context::FFIUriResolutionContext,
    uri_package_or_wrapper::FFIUriPackageOrWrapper,
};

#[derive(Debug)]
pub struct FFIStaticUriResolver {
    inner_resolver: StaticResolver,
}

impl FFIStaticUriResolver {
    pub fn new(uri_map: HashMap<String, Box<dyn FFIUriPackageOrWrapper>>) -> FFIStaticUriResolver {
        let uri_map: HashMap<String, UriPackageOrWrapper> = uri_map
            .into_iter()
            .map(|(uri, variant)| {
                let uri_package_or_wrapper: UriPackageOrWrapper = variant.into();
                (uri, uri_package_or_wrapper)
            })
            .collect();

        FFIStaticUriResolver {
            inner_resolver: StaticResolver::new(uri_map),
        }
    }
}

impl FFIUriResolver for FFIStaticUriResolver {
    fn try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        invoker: Box<dyn FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Result<Box<dyn FFIUriPackageOrWrapper>, FFIError> {
        let result = self.inner_resolver.try_resolve_uri(
            &uri.0,
            Arc::new(FFIInvokerWrapping(invoker)),
            resolution_context.0.clone(),
        )?;

        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, sync::Arc};

    use polywrap_tests_utils::mocks::{get_mock_invoker, get_mock_uri_package_or_wrapper};

    use crate::{
        invoker::InvokerWrapping,
        resolvers::{
            ffi_resolver::FFIUriResolver,
            resolution_context::FFIUriResolutionContext,
            uri_package_or_wrapper::{FFIUriPackageOrWrapper, FFIUriPackageOrWrapperKind},
        },
        uri::FFIUri,
    };

    use super::FFIStaticUriResolver;

    #[test]
    fn ff_try_resolver_uri() {
        let mock_uri_package_or_wrapper = get_mock_uri_package_or_wrapper();
        let ffi_uri = Arc::new(FFIUri::from_string("wrap/mock"));

        let ffi_uri_package_or_wrapper: Box<dyn FFIUriPackageOrWrapper> =
            Box::new(mock_uri_package_or_wrapper);
        let ffi_static_resolver = FFIStaticUriResolver::new(HashMap::from([(
            ffi_uri.0.to_string(),
            ffi_uri_package_or_wrapper,
        )]));

        let ffi_uri_resolution_context = Arc::new(FFIUriResolutionContext::new());

        let response = ffi_static_resolver.try_resolve_uri(
            ffi_uri,
            Box::new(InvokerWrapping(get_mock_invoker())),
            ffi_uri_resolution_context,
        );

        let response = response.unwrap();
        let kind = response.get_kind();

        match kind {
            FFIUriPackageOrWrapperKind::WRAPPER => {
                let wrapper = response.as_wrapper().get_wrapper();
                let response = wrapper.invoke(
                    "foo".to_string(),
                    None,
                    None,
                    Box::new(InvokerWrapping(get_mock_invoker())),
                    None,
                );
                assert_eq!(response.unwrap(), [195])
            }
            _ => {
                panic!("Kind was expected to be wrapper but received: {:?}", kind)
            }
        }
    }
}
