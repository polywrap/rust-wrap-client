use crate::wrap::types::{Request, Response, ResponseType};
use polywrap_plugin::error::PluginError;
use std::{collections::BTreeMap};

pub enum RequestMethod {
    GET,
    POST,
}

pub async fn parse_response(response: reqwest::Response, encoding: ResponseType) -> Result<Response, PluginError> {
    let headers = response
        .headers()
        .iter()
        .map(|(name, value)| {
            (
              name.to_string(),
              value.to_str().unwrap().to_string(),
            )
        })
        .collect::<BTreeMap<String, String>>();

    let status = response.status();
    let status_text = status.canonical_reason().unwrap().to_string();
    
    let data = response
        .bytes().await
        .map_err(|e| PluginError::ModuleError(e.to_string()))?;

    let data = match encoding {
        ResponseType::BINARY => base64::encode(data),
        _ => String::from_utf8_lossy(&data).to_string()
    };

    Ok(Response {
        status: status.as_u16().into(),
        status_text,
        headers: Some(headers),
        body: Some(data),
    })
}

pub fn parse_request(
    url: &str,
    request: Option<Request>,
    method: RequestMethod,
) -> Result<reqwest::RequestBuilder, PluginError> {
    let http_client = reqwest::Client::new();
    let mut request_builder = match method {
        RequestMethod::GET => http_client.get(url),
        RequestMethod::POST => http_client.post(url),
    };

    if let Some(request) = request {
        if let Some(url_params) = request.url_params {
            let query_params: Vec<(String, String)> = url_params
                .iter()
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect();
            request_builder = request_builder.query(&query_params)
        };

        if let Some(headers) = request.headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.header(name, value)
            }
        }
    }

    Ok(request_builder)
}
