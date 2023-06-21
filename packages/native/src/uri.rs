use polywrap_client::core::uri::Uri;

#[derive(Clone, Debug, Hash, Eq)]
pub struct FFIUri(pub Uri);

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

    pub fn from_string(uri: &str) -> Self {
        FFIUri(Uri::try_from(uri).unwrap())
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
        write!(f, "{}", self.0.to_string())
    }
}

#[cfg(test)]
mod test {
    use polywrap_client::{core::uri::Uri, macros::uri};

    use crate::uri::FFIUri;

    #[test]
    pub fn string_into_ffi_uri() {
        let string_uri = "mock/a";
        let uri: FFIUri = string_uri.try_into().unwrap();
        assert_eq!(FFIUri::from_string("wrap://mock/a"), uri);
    }

    #[test]
    pub fn ffi_uri_to_string() {
        let uri = FFIUri::from_string("wrap://mock/a");
        assert_eq!(uri.to_string(), "wrap://mock/a");
    }

    #[test]
    pub fn ffi_uri_from_string() {
        let expected_uri = FFIUri::from_string("wrap://mock/a");
        let str_uri = "mock/a";
        let uri = FFIUri::try_from(str_uri).unwrap();
        assert_eq!(uri, expected_uri);

        let string_uri = String::from("mock/a");
        let uri = FFIUri::try_from(string_uri).unwrap();
        assert_eq!(uri, expected_uri);
    }

    #[test]
    pub fn string_from_ffi_uri() {
        let uri = FFIUri::from_string("wrap://mock/a");
        let string_uri = String::from(uri);
        assert_eq!("wrap://mock/a".to_string(), string_uri);
    }

    #[test]
    pub fn uri_ffi_from_uri() {
        let uri = uri!("mock/a");
        let expected_ffi_uri = FFIUri::from_string("wrap://mock/a");
        assert_eq!(FFIUri::from(uri), expected_ffi_uri);
    }

    #[test]
    pub fn uri_from_ffi_uri() {
        let ffi_uri = FFIUri::from_string("wrap://mock/a");
        let expected_uri = uri!("mock/a");
        assert_eq!(Uri::from(ffi_uri), expected_uri);
    }
}
