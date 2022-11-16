use std::{collections::HashMap, sync::Arc};

use polywrap_core::invoke::Invoker;
use polywrap_plugin::{impl_plugin_module, module::PluginModule};

fn parse_response(response: ureq::Response) -> Result<HttpResponse, polywrap_core::error::Error> {
    let headers = response
        .headers_names()
        .iter()
        .map(|header_name| {
            (
                header_name.to_string(),
                response.header(header_name).unwrap().to_string(),
            )
        })
        .collect::<HashMap<String, String>>();

    let status = (&response).status();
    let status_text = (&response).status_text().to_string();

    let data = response
        .into_string()
        .map_err(|e| polywrap_core::error::Error::InvokeError(e.to_string()))?;

    Ok(HttpResponse {
        status,
        status_text,
        headers: Some(headers),
        body: Some(data.to_string()),
    })
}

fn parse_request(request: HttpRequest) -> Result<ureq::Request, polywrap_core::error::Error> {
  
}

pub struct HttpPlugin {}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub enum HttpResponseType {
    Text,
    Binary,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct HttpRequest {
    pub headers: Option<HashMap<String, String>>,
    pub url_params: Option<HashMap<String, String>>,
    pub response_type: HttpResponseType,
    pub body: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Args {
    pub url: String,
    pub request: HttpRequest,
}

impl HttpPlugin {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(
        &self,
        args: &Args,
        _: Arc<dyn Invoker>,
    ) -> Result<HttpResponse, polywrap_core::error::Error> {
        let response = ureq::get(&args.url)
            .call()
            .map_err(|e| polywrap_core::error::Error::InvokeError(e.to_string()))?;

        parse_response(response)
    }

    pub fn post(
        &self,
        args: &Args,
        _: Arc<dyn Invoker>,
    ) -> Result<HttpResponse, polywrap_core::error::Error> {
        let response = ureq::get(&args.url)
            .call()
            .map_err(|e| polywrap_core::error::Error::InvokeError(e.to_string()))?;

        parse_response(response)
    }
}

impl_plugin_module!(HttpPlugin, (get, Args), (post, Args));
