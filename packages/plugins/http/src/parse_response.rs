use std::collections::BTreeMap;
use std::io::Read;

use crate::wrap::types::{Response, ResponseType};
use polywrap_plugin::{error::PluginError, Map};

pub fn parse_response(
    response: ureq::Response,
    encoding: ResponseType,
) -> Result<Response, PluginError> {
    let headers = response
        .headers_names()
        .iter()
        .map(|header_name| {
            let header_value = response
                .header(header_name)
                .ok_or(ParseResponseError::HeaderNotFound(header_name.to_string()))?;
            Ok((header_name.to_string(), header_value.to_string()))
        })
        .collect::<Result<BTreeMap<String, String>, ParseResponseError>>()?;
    let status = response.status();
    let status_text = response.status_text().to_string();

    let mut reader = response.into_reader();
    let mut data = vec![];
    reader
        .read_to_end(&mut data)
        .map_err(ParseResponseError::ReadResponseBodyError)?;

    let data = match encoding {
        ResponseType::BINARY => base64::encode(data),
        _ => String::from_utf8_lossy(&data).to_string(),
    };

    Ok(Response {
        status: status.into(),
        status_text,
        headers: Some(headers),
        body: Some(data),
    })
}

#[derive(thiserror::Error, Debug)]
pub enum ParseResponseError {
    #[error("Error reading response body: `{0}`")]
    ReadResponseBodyError(std::io::Error),
    #[error("Header not found: `{0}`")]
    HeaderNotFound(String),
}

impl From<ParseResponseError> for PluginError {
    fn from(e: ParseResponseError) -> Self {
        PluginError::InvocationError {
            exception: e.to_string(),
        }
    }
}
