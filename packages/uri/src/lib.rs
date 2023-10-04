use std::{fmt::Display, str::FromStr};

use regex::Regex;
use serde::{Deserialize, Serialize};

/// Represents a Wrap URI.
#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq)]
pub struct Uri {
    /// The URI's authority.
    authority: String,
    /// The URI's path.
    path: String,
    /// A string representation of the full Wrap URI.
    uri: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError(pub String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl std::error::Error for ParseError {}

impl Uri {
    fn try_from_string(uri: &str) -> Result<Uri, ParseError> {
        let mut processed = uri.to_string();

        while processed.starts_with('/') {
            processed = processed[1..].to_string();
        }

        let wrap_scheme_idx = processed.find("wrap://");

        if wrap_scheme_idx.is_none() {
            processed = format!("wrap://{processed}");
        }

        if wrap_scheme_idx.is_some() && wrap_scheme_idx.unwrap() != 0 {
            return Err(ParseError(String::from(
                "The wrap:// scheme must be at the beginning of the URI string".to_string(),
            )));
        }

        let reg = Regex::new("wrap://([a-z][a-z0-9-_.]+)/(.*)").unwrap();

        let captures = reg.captures(&processed);

        if captures.as_ref().is_none() || captures.as_ref().unwrap().len() != 3 {
            return Err(ParseError(format!(
                r#"URI is malformed, here are some examples of valid URIs:
            wrap://ipfs/QmHASH
            wrap://wrapscan.io/polywrap/cool-wrap
            wrapscan.io/user/wrap
            wrap://ens/domain.eth
            Invalid URI Received: {uri}"#,
            )));
        }

        let result = captures.unwrap();

        Ok(Uri {
            authority: result[1].to_string(),
            path: result[2].to_string(),
            uri: processed.to_string(),
        })
    }

    /// Construct a Wrap URI from its parts.
    /// 
    /// # Safety
    /// This blindly builds the URI from its parts without performing any checks.
    /// You need to ensure that all parts represent the same, valid Wrap URI.
    pub unsafe fn from_parts(authority: String, path: String, uri: String) -> Uri {
        Uri {
            authority: authority,
            path: path,
            uri: uri,
        }
    }

    /// Return the URI's authority.
    pub fn authority(&self) -> &str {
        &self.authority
    }

    /// Return the URI's path.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Return the URI as a string.
    pub fn uri(&self) -> &str {
        &self.uri
    }
}

impl PartialEq for Uri {
    fn eq(&self, other: &Self) -> bool {
        self.uri == other.uri
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}

impl From<Uri> for String {
    fn from(uri: Uri) -> Self {
        uri.to_string()
    }
}

impl TryFrom<String> for Uri {
    type Error = ParseError;

    fn try_from(uri: String) -> Result<Self, Self::Error> {
        Uri::try_from_string(&uri)
    }
}

impl TryFrom<&str> for Uri {
    type Error = ParseError;

    fn try_from(uri: &str) -> Result<Self, Self::Error> {
        Uri::try_from_string(uri)
    }
}

impl FromStr for Uri {
    type Err = ParseError;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        Uri::try_from_string(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn try_from_string_valid() {
        assert!(Uri::try_from_string("wrap://ipfs/QmHASH").is_ok());
        assert!(Uri::try_from_string("////wrap://ipfs/QmHASH").is_ok());
        assert!(Uri::try_from_string("wrapscan.io/user/cool-wrap").is_ok());
    }

    #[test]
    fn try_from_string_invalid() {
        assert!(Uri::try_from_string("wraps://ipfs/QmHASH").is_err());
        assert!(Uri::try_from_string("ipfs/QmHASHwrap://").is_err());
        assert!(Uri::try_from_string("").is_err());
    }

    #[test]
    fn from_parts() {
        let uri =
            unsafe { Uri::from_parts("authority".to_owned(), "path".to_owned(), "uri".to_owned()) };
        assert_eq!(uri.authority(), "authority");
        assert_eq!(uri.path(), "path");
        assert_eq!(uri.uri(), "uri");
    }

    #[test]
    fn equality() {
        let (uri1, uri2, uri3) = unsafe {
            (
                Uri::from_parts("authority".to_owned(), "path".to_owned(), "uri".to_owned()),
                Uri::from_parts("authority".to_owned(), "path".to_owned(), "uri".to_owned()),
                Uri::from_parts(
                    "authority".to_owned(),
                    "path".to_owned(),
                    "different".to_owned(),
                ),
            )
        };

        assert_eq!(uri1, uri2);
        assert_ne!(uri1, uri3);
    }

    #[test]
    fn from() {
        let uri = Uri::try_from_string("wrap://auth/path").unwrap();
        let string: String = uri.into();
        assert_eq!(string, "wrap://auth/path");
    }

    #[test]
    fn string_try_into() {
        let uri: Result<Uri, ParseError> = "wrap://ipfs/QmHASH".try_into();
        assert!(uri.is_ok());

        let bad_uri: Result<Uri, ParseError> = "bad_uri".try_into();
        assert!(bad_uri.is_err());
    }

    #[test]
    fn parse_str() {
        let uri: Uri = "wrap://ipfs/QmHASH".parse().unwrap();
        assert_eq!(uri.uri(), "wrap://ipfs/QmHASH");

        let bad_uri: Result<Uri, ParseError> = "bad_uri".parse();
        assert!(bad_uri.is_err());
    }

    #[test]
    fn display() {
        let uri = Uri::try_from_string("wrap://authority/uri").unwrap();
        assert_eq!(format!("{}", uri), "wrap://authority/uri");
    }
}
