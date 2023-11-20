use std::{str::FromStr, sync::Arc};

use polywrap_client::core::uri::{ParseError, Uri};

use crate::error::FFIError;

#[derive(Clone, Debug, Hash, Eq)]
pub struct FFIUri(pub Uri);

pub fn ffi_uri_from_string(uri: &str) -> Result<Arc<FFIUri>, FFIError> {
    let uri: Result<Uri, ParseError> = uri.parse();
    uri.map_err(|e| FFIError::UriParseError { err: e.to_string() })
        .map(|v| Arc::new(FFIUri(v)))
}

impl FFIUri {
    pub fn new(authority: &str, path: &str, uri: &str) -> Self {
        unsafe {
            FFIUri(Uri::from_parts(
                authority.to_owned(),
                path.to_owned(),
                uri.to_owned(),
            ))
        }
    }

    pub fn authority(&self) -> String {
        self.0.authority().to_string()
    }

    pub fn path(&self) -> String {
        self.0.path().to_string()
    }

    pub fn to_string_uri(&self) -> String {
        self.0.to_string()
    }
}

impl PartialEq for FFIUri {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl From<FFIUri> for String {
    fn from(uri: FFIUri) -> Self {
        uri.to_string_uri()
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
        write!(f, "{}", self.0.to_string())
    }
}

impl FromStr for FFIUri {
    type Err = polywrap_client::core::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri: Uri = s.try_into()?;
        Ok(FFIUri(uri))
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use polywrap_client::core::{macros::uri, uri::Uri};

    use crate::uri::{ffi_uri_from_string, FFIUri};

    #[test]
    pub fn string_into_ffi_uri() {
        let string_uri = "mock/a";
        let uri: FFIUri = string_uri.into();
        assert_eq!(ffi_uri_from_string("wrap://mock/a").unwrap(), Arc::new(uri));
    }

    #[test]
    pub fn ffi_uri_to_string() {
        let uri = ffi_uri_from_string("wrap://mock/a");
        assert_eq!(uri.unwrap().to_string(), "wrap://mock/a");
    }

    #[test]
    pub fn ffi_uri_from_string_test() {
        let expected_uri = ffi_uri_from_string("wrap://mock/a");
        let str_uri = "mock/a";
        let uri: FFIUri = str_uri.parse().unwrap();
        assert_eq!(Arc::new(uri), expected_uri.clone().unwrap());

        let string_uri = String::from("mock/a");
        let uri: FFIUri = string_uri.parse().unwrap();
        assert_eq!(Arc::new(uri), expected_uri.unwrap());
    }

    #[test]
    pub fn string_from_ffi_uri() {
        let uri = ffi_uri_from_string("wrap://mock/a");
        let string_uri = String::from(uri.unwrap().0.uri());
        assert_eq!("wrap://mock/a".to_string(), string_uri);
    }

    #[test]
    pub fn uri_ffi_from_uri() {
        let uri = uri!("mock/a");
        let expected_ffi_uri = ffi_uri_from_string("wrap://mock/a");
        assert_eq!(Arc::new(FFIUri::from(uri)), expected_ffi_uri.unwrap());
    }

    #[test]
    pub fn uri_from_ffi_uri() {
        let ffi_uri = ffi_uri_from_string("wrap://mock/a");
        let expected_uri = uri!("mock/a");
        assert_eq!(ffi_uri.unwrap().0, expected_uri);
    }

    #[test]
    pub fn getters() {
        let ffi_uri = ffi_uri_from_string("mock/a").unwrap();

        assert_eq!(ffi_uri.authority(), "mock");
        assert_eq!(ffi_uri.path(), "a");
        assert_eq!(ffi_uri.to_string(), "wrap://mock/a");
    }
}
