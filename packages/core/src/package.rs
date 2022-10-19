use std::{future::Future, pin::Pin};

use crate::{wrapper::Wrapper, error::CoreError};

pub trait WrapPackage {
  fn create_wrapper(&self) -> Pin<Box<dyn Future< Output = Result<Box<dyn Wrapper>, CoreError>>>>;
}