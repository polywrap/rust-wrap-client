use std::{fmt::Debug, sync::Arc};

use polywrap_core::{
    invoker::Invoker,
    wrapper::{GetFileOptions, Wrapper},
};

#[derive(Debug)]
pub struct MockWrapper;
#[derive(Debug)]
pub struct DifferentMockWrapper;

impl Wrapper for MockWrapper {
    fn invoke(
        &self,
        method: &str,
        _: Option<&[u8]>,
        _: Option<&[u8]>,
        _: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        // In Msgpack: True = [195] and False = [194]
        if method == "foo" {
            Ok(vec![195])
        } else {
            Ok(vec![194])
        }
    }

    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![1])
    }
}

impl Wrapper for DifferentMockWrapper {
    fn invoke(
        &self,
        method: &str,
        _: Option<&[u8]>,
        _: Option<&[u8]>,
        _: Arc<dyn Invoker>,
    ) -> Result<Vec<u8>, polywrap_core::error::Error> {
        // In Msgpack: True = [195] and False = [194]
        if method == "bar" {
            Ok(vec![195])
        } else {
            Ok(vec![194])
        }
    }

    fn get_file(&self, _: &GetFileOptions) -> Result<Vec<u8>, polywrap_core::error::Error> {
        Ok(vec![1])
    }
}

pub fn get_mock_wrapper() -> Arc<dyn Wrapper> {
    Arc::new(MockWrapper {})
}

pub fn get_different_mock_wrapper() -> Arc<dyn Wrapper> {
    Arc::new(DifferentMockWrapper {})
}

