use std::{path::{Path,PathBuf},sync::{Arc}, fmt::{Debug, Formatter}};
use async_trait::async_trait;
use futures::lock::Mutex;

use polywrap_core::{wrapper::{Wrapper, GetFileOptions},  invoke::Invoker, uri::Uri, env::Env, resolvers::uri_resolution_context::UriResolutionContext, package::WrapPackage};
use wrap_manifest_schemas::versions::WrapManifest;

pub struct MockWrapper;
pub struct MockPackage;

impl Debug for MockWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockWrapper")
    }
}

impl MockWrapper {
    pub fn new() -> Self {
        Self {}
    }
}


impl Debug for MockPackage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MockPackage")
    }
}

impl MockPackage {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl WrapPackage for MockPackage {
    async fn create_wrapper(&self) -> Result<Arc<Mutex<dyn Wrapper>>, polywrap_core::error::Error> {
        Ok(Arc::new(Mutex::new(get_mock_wrapper())))
    }

    async fn get_manifest(
        &self, 
        _: Option<polywrap_core::package::GetManifestOptions>
    ) ->  Result<WrapManifest, polywrap_core::error::Error> {
        unimplemented!()
    }
}

#[async_trait]
impl Wrapper for MockWrapper {
    async fn invoke(
        &mut self,
        _: Arc<dyn Invoker>,
        _: &Uri,
        _: &str,
        _: Option< &[u8]>,
        _: Option<Env>,
        _: Option<&mut UriResolutionContext>
    ) ->  Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![2])
    }

    async fn get_file(
        &self,
        _: &GetFileOptions
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![2])
    }
}

pub fn get_mock_wrapper() -> MockWrapper {
    MockWrapper::new()
}

pub fn get_tests_path() -> Result<PathBuf, ()> {
    let path = Path::new("../../packages/tests-utils/cases").canonicalize().unwrap();
    Ok(path)
}