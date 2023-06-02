use polywrap_client::core::uri::Uri;

#[derive(Debug)]
pub struct FFIUri(pub Uri);

impl FFIUri {
  pub fn new(authority: &str, path: &str, uri: &str) -> Self {
    FFIUri(Uri {
      authority: authority.to_string(),
      path: path.to_string(),
      uri: uri.to_string()
     })
  }

  pub fn from_string(uri: &str) -> Self {
    FFIUri(Uri::new(uri))
  }

  pub fn to_string_uri(&self) -> String {
    self.0.to_string()
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

impl From<FFIUri> for Uri {
  fn from(uri: FFIUri) -> Self {
      uri.0
  }
}

impl From<Uri> for FFIUri {
  fn from(uri: Uri) -> Self {
      FFIUri(uri)
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