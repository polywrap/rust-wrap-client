use crate::error::CoreError;
use crate::uri::uri::Uri;
use crate::uri::uri_resolver::UriResolver;
use async_trait::async_trait;
use crate::wrapper::{GetFileOptions};

pub struct UriRedirect {
  pub from: Uri,
  pub to: Uri,
}

pub struct ClientConfig {
  pub redirects: Vec<UriRedirect>,
  pub resolver: Box<dyn UriResolver>
}

#[async_trait(?Send)]
pub trait Client {
  fn get_config(&self) -> &ClientConfig;
  fn get_redirects(&self) -> &Vec<UriRedirect>;
  fn get_uri_resolver(&self) -> &Box<dyn UriResolver>;
  async fn get_file(&mut self, uri: Uri, options: GetFileOptions) -> Result<String, CoreError>;
}
