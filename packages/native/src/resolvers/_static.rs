use polywrap_client::{
    core::{
        resolution::{uri_resolution_context::UriPackageOrWrapper, uri_resolver::UriResolver},
        uri::Uri,
    },
    resolvers::static_resolver::StaticResolver,
};
use std::{collections::HashMap, sync::Arc};

use crate::{error::FFIError, invoker::FFIInvoker, uri::FFIUri};

use super::{
    ffi_resolver::IFFIUriResolver, resolution_context::FFIUriResolutionContext,
    uri_package_or_wrapper::FFIUriPackageOrWrapper,
};

#[derive(Debug)]
pub struct FFIStaticUriResolver(StaticResolver);

impl FFIStaticUriResolver {
    pub fn new(
        uri_map: HashMap<String, Arc<FFIUriPackageOrWrapper>>,
    ) -> Result<FFIStaticUriResolver, FFIError> {
        let uri_map: Result<HashMap<Uri, UriPackageOrWrapper>, _> = uri_map
            .into_iter()
            .map(|(uri, variant)| {
                uri.parse::<Uri>()
                    .map_err(|e| FFIError::UriParseError { err: e.to_string() })
                    .map(|uri| {
                        let uri_package_or_wrapper: UriPackageOrWrapper = variant.as_ref().0.clone();
                        (uri, uri_package_or_wrapper)
                    })
            })
            .collect(); // collect into a Result

        // propagate error if the conversion failed
        let uri_map = uri_map?;

        Ok(FFIStaticUriResolver(StaticResolver::new(uri_map)))
    }

    pub fn try_resolve_uri(
      &self,
      uri: Arc<FFIUri>,
      invoker: Arc<FFIInvoker>,
      resolution_context: Arc<FFIUriResolutionContext>,
  ) -> Result<Arc<FFIUriPackageOrWrapper>, FFIError> {
      IFFIUriResolver::try_resolve_uri(self, uri, invoker, resolution_context)
  }
}

impl IFFIUriResolver for FFIStaticUriResolver {
    fn try_resolve_uri(
        &self,
        uri: Arc<FFIUri>,
        invoker: Arc<FFIInvoker>,
        resolution_context: Arc<FFIUriResolutionContext>,
    ) -> Result<Arc<FFIUriPackageOrWrapper>, FFIError> {
        let result = self.0.try_resolve_uri(
            &uri.0,
            invoker.0.clone(),
            resolution_context.0.clone(),
        )?;

        Ok(Arc::new(FFIUriPackageOrWrapper(result)))
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, sync::Arc};

    use polywrap_tests_utils::mocks::{get_mock_invoker, get_mock_uri_package_or_wrapper};

    use crate::{
        invoker::FFIInvoker,
        resolvers::{
            resolution_context::FFIUriResolutionContext,
            uri_package_or_wrapper::{FFIUriPackageOrWrapper, FFIUriPackageOrWrapperKind},
        },
        uri::ffi_uri_from_string, wrapper::FFIWrapper,
    };

    use super::FFIStaticUriResolver;

    #[test]
    fn ffi_static_resolver_returns_error_with_bad_uri() {
        let mock_uri_package_or_wrapper = get_mock_uri_package_or_wrapper();

        let ffi_uri_package_or_wrapper = Arc::new(FFIUriPackageOrWrapper(mock_uri_package_or_wrapper));
        let ffi_static_resolver = FFIStaticUriResolver::new(HashMap::from([(
            "wrong-uri-format".to_string(),
            ffi_uri_package_or_wrapper,
        )]));

        assert!(ffi_static_resolver.is_err());
        assert!(ffi_static_resolver
            .unwrap_err()
            .to_string()
            .contains("Error parsing URI:"));
    }

    #[test]
    fn ffi_try_resolver_uri() {
        let mock_uri_package_or_wrapper = get_mock_uri_package_or_wrapper();
        let ffi_uri = ffi_uri_from_string("wrap/mock").unwrap();

        let ffi_uri_package_or_wrapper =
            Arc::new(FFIUriPackageOrWrapper(mock_uri_package_or_wrapper));
        let ffi_static_resolver = FFIStaticUriResolver::new(HashMap::from([(
            "wrap/mock".to_string(),
            ffi_uri_package_or_wrapper,
        )]))
        .unwrap();

        let ffi_uri_resolution_context = Arc::new(FFIUriResolutionContext::new());

        let response = ffi_static_resolver.try_resolve_uri(
            ffi_uri,
            Arc::new(FFIInvoker(get_mock_invoker())),
            ffi_uri_resolution_context,
        );

        let response = response.unwrap();
        let kind = response.get_kind();

        match kind {
            FFIUriPackageOrWrapperKind::WRAPPER => {
                let wrapper = FFIWrapper(response.as_wrapper().unwrap().wrapper);
                let response = wrapper.invoke(
                    "foo",
                    None,
                    None,
                    Arc::new(FFIInvoker(get_mock_invoker())),
                );
                assert_eq!(response.unwrap(), [195])
            }
            _ => {
                panic!("Kind was expected to be wrapper but received: {:?}", kind)
            }
        }
    }
}
