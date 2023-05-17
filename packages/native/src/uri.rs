use polywrap_client::core::uri::Uri;

pub struct FFIUri(Uri);

impl FFIUri {
  pub fn new(uri: &str) -> Self {
    FFIUri(Uri::new(uri))
  }
}

impl PartialEq for FFIUri {
  fn eq(&self, other: &Self) -> bool {
      self.0.uri == other.0.uri
  }
}

impl From<FFIUri> for String {
  fn from(uri: FFIUri) -> Self {
      uri.to_string()
  }
}

impl TryFrom<String> for FFIUri {
  type Error = polywrap_client::core::error::Error;

  fn try_from(uri: String) -> Result<Self, Self::Error> {
    let uri: Uri = uri.try_into()?;
    Ok(FFIUri(uri))
  }
}

impl TryFrom<&str> for FFIUri {
  type Error = polywrap_client::core::error::Error;

  fn try_from(uri: &str) -> Result<Self, Self::Error> {
    let uri: Uri = uri.try_into()?;
    Ok(FFIUri(uri))
  }
}

impl std::fmt::Display for FFIUri {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "{}", self.0.uri)
  }
}