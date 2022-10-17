use super::error::CoreError;
use regex::Regex;

pub struct Uri {
    authority: String,
    path: String,
    uri: String,
}

impl Uri {
    pub fn new(uri: &str) -> Self {
        let parsed_uri = Uri::from_string(uri);

        if parsed_uri.is_err() {
            panic!("Error parsing URI: `{}`", uri);
        } else {
            parsed_uri.unwrap()
        }
    }

    // TODO: compare uri === uri

    pub fn from_string(uri: &str) -> Result<Uri, CoreError> {
        let mut processed = uri.clone().to_string();

        while processed.chars().nth(0).unwrap() == '/' {
            processed = processed[1..].to_string();
        }

        let wrap_scheme_idx = processed.find("wrap://");

        if wrap_scheme_idx.is_none() {
            processed = format!("wrap://{}", processed);
        }

        if wrap_scheme_idx.is_some() && wrap_scheme_idx.unwrap() != 0 {
            return Err(CoreError::UriParseError(format!(
                "The wrap:// scheme must be at the beginning of the URI string"
            )));
        }

        let reg = Regex::new(
            r"(?x)
            ^(?P<login>[^@\s]+)@
            ([[:word:]]+\.)*
            [[:word:]]+$
            ",
        )
        .unwrap();

        let captures = reg.captures(&processed);

        if captures.as_ref().is_none() || captures.as_ref().unwrap().len() != 3 {
            return Err(CoreError::UriParseError(format!(
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

    pub fn to_string(&self) -> String {
        self.uri.to_string()
    }
}

impl From<String> for Uri {
    fn from(uri: String) -> Self {
        Uri::new(&uri)
    }
}

impl Into<String> for Uri {
    fn into(self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for Uri {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}
