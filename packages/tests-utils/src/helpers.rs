use std::{path::{Path,PathBuf},sync::Arc, fmt::{Debug, Formatter}};
use async_trait::async_trait;

use polywrap_core::{wrapper::{Wrapper, GetFileOptions},  invoke::Invoker, uri::Uri, env::Env, resolvers::uri_resolution_context::UriResolutionContext};

pub struct MockWrapper;

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