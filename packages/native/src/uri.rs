use polywrap_client::core::uri::Uri;

#[derive(Debug)]
pub struct FFIUri(pub Uri);

impl FFIUri {
    pub fn new(authority: &str, path: &str, uri: &str) -> Self {
        FFIUri(Uri {
            authority: authority.to_string(),
            path: path.to_string(),
            uri: uri.to_string(),
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

mod test {
    use polywrap_client::core::uri::Uri;

    use crate::uri::FFIUri;

    #[test]
    pub fn string_into_ffi_uri() {
        let string_uri = "mock/a";
        let uri: FFIUri = string_uri.try_into().unwrap();
        assert_eq!(FFIUri::new("mock", "a", "wrap://mock/a"), uri);
    }

    #[test]
    pub fn ffi_uri_to_string() {
        let uri = FFIUri::new("mock", "a", "wrap://mock/a");
        assert_eq!(uri.to_string(), "wrap://mock/a");
    }

    #[test]
    pub fn ffi_uri_from_string() {
        let expected_uri = FFIUri::new("mock", "a", "wrap://mock/a");
        let str_uri = "mock/a";
        let uri = FFIUri::try_from(str_uri).unwrap();
        assert_eq!(uri, expected_uri);

        let string_uri = String::from("mock/a");
        let uri = FFIUri::try_from(string_uri).unwrap();
        assert_eq!(uri, expected_uri);
    }

    #[test]
    pub fn string_from_ffi_uri() {
        let uri = FFIUri::new("mock", "a", "wrap://mock/a");
        let string_uri = String::from(uri);
        assert_eq!("wrap://mock/a".to_string(), string_uri);
    }

    #[test]
    pub fn uri_ffi_from_uri() {
        let uri = Uri::new("mock/a");
        let expected_ffi_uri = FFIUri::new("mock", "a", "wrap://mock/a");
        assert_eq!(FFIUri::from(uri), expected_ffi_uri);
    }

    #[test]
    pub fn uri_from_ffi_uri() {
        let ffi_uri = FFIUri::new("mock", "a", "wrap://mock/a");
        let expected_uri = Uri::new("mock/a");
        assert_eq!(Uri::from(ffi_uri), expected_uri);
    }
}
