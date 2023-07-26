use std::{fmt::Debug, sync::Arc};

use crate::{
    error::FFIError,
    invoker::FFIInvoker,
    wrapper::{FFIWrapper, IFFIWrapper},
};

#[derive(Debug)]
pub struct MockWrapper;
#[derive(Debug)]
pub struct DifferentMockWrapper;

impl IFFIWrapper for MockWrapper {
    fn invoke(
        &self,
        method: String,
        _: Option<Vec<u8>>,
        _: Option<Vec<u8>>,
        _: Arc<FFIInvoker>,
    ) -> Result<Vec<u8>, FFIError> {
        // In Msgpack: True = [195] and False = [194]
        if method == "foo" {
            Ok(vec![195])
        } else {
            Ok(vec![194])
        }
    }
}

impl IFFIWrapper for DifferentMockWrapper {
    fn invoke(
        &self,
        method: String,
        _: Option<Vec<u8>>,
        _: Option<Vec<u8>>,
        _: Arc<FFIInvoker>,
    ) -> Result<Vec<u8>, FFIError> {
        // In Msgpack: True = [195] and False = [194]
        if method == "bar" {
            Ok(vec![195])
        } else {
            Ok(vec![194])
        }
    }
}

pub fn get_mock_ffi_wrapper() -> FFIWrapper {
    FFIWrapper(Box::new(MockWrapper {}))
}

pub fn get_different_mock_ffi_wrapper() -> FFIWrapper {
    FFIWrapper(Box::new(DifferentMockWrapper {}))
}
