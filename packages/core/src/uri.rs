use crate::error::Error;
use serde::Serialize;
use regex::Regex;

#[derive(Clone,Serialize,Debug)]
pub struct Uri {
    pub authority: String,
    pub path: String,
    pub uri: String,
}

impl Uri {
    pub fn new(uri: &str) -> Self {
        let parsed_uri = Uri::from_string(uri);

        if let Ok(result_uri) = parsed_uri {
          result_uri
        } else {
            panic!("Error parsing URI: `{}`", uri);
        }
    }

    fn from_string(uri: &str) -> Result<Uri, Error> {
        let mut processed = uri.to_string();

        while processed.starts_with('/') {
            processed = processed[1..].to_string();
        }

        let wrap_scheme_idx = processed.find("wrap://");

        if wrap_scheme_idx.is_none() {
            processed = format!("wrap://{}", processed);
        }

        if wrap_scheme_idx.is_some() && wrap_scheme_idx.unwrap() != 0 {
            return Err(Error::UriParseError("The wrap:// scheme must be at the beginning of the URI string".to_string()));
        }

        let reg = Regex::new(
            "wrap://([a-z][a-z0-9-_]+)/(.*)",
        )
        .unwrap();

        let captures = reg.captures(&processed);

        if captures.as_ref().is_none() || captures.as_ref().unwrap().len() != 3 {
            return Err(Error::UriParseError(format!(
                r#"URI is malformed, here are some examples of valid URIs:
            wrap://ipfs/QmHASH
            wrap://ens/domain.eth
            ens/domain.eth
            Invalid URI Received: ${}"#,
                uri
            )));
        }

        let result = captures.unwrap();

        Ok(Uri {
            authority: result[1].to_string(),
            path: result[2].to_string(),
            uri: processed.to_string(),
        })
    }
}

impl PartialEq for Uri {
    fn eq(&self, other: &Self) -> bool {
        self.uri == other.uri
    }
}

impl From<Uri> for String {
    fn from(uri: Uri) -> Self {
        uri.uri
    }
}

impl TryFrom<String> for Uri {
    type Error = Error;

    fn try_from(uri: String) -> Result<Self, Self::Error> {
        Uri::from_string(&uri)
    }
}

impl TryFrom<&str> for Uri {
  type Error = Error;

  fn try_from(uri: &str) -> Result<Self, Self::Error> {
      Uri::from_string(uri)
  }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}
