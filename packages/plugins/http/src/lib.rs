use crate::{
    parse_request::parse_request, parse_response::parse_response, wrap::wrap_info::get_manifest,
};
use multipart::client::lazy::Multipart;
use polywrap_plugin::*;
use std::{io::Cursor, sync::Arc};
use ureq::{Request as UreqRequest, Response as UreqResponse};
use wrap::{
    module::{ArgsGet, ArgsPost, Module},
    types::{FormDataEntry, Response, ResponseType},
};
pub mod parse_request;
pub mod parse_response;
pub mod wrap;
use std::result::Result;

pub enum RequestMethod {
    GET,
    POST,
}

#[derive(Debug)]
pub struct HttpPlugin;

#[plugin_impl]
impl Module for HttpPlugin {
    fn get(
        &mut self,
        args: &ArgsGet,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<Response>, PluginError> {
        let response = parse_request(&args.url, args.request.clone(), RequestMethod::GET)
            .call()
            .map_err(|e| HttpPluginError::SendRequestError(e.to_string()))?;

        let response_type = if let Some(r) = &args.request {
            r.response_type
        } else {
            ResponseType::TEXT
        };

        let parsed_response = parse_response(response, response_type)?;

        Ok(Some(parsed_response))
    }

    fn post(
        &mut self,
        args: &ArgsPost,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<Response>, PluginError> {
        let request = parse_request(&args.url, args.request.clone(), RequestMethod::POST);

        let response_type = if let Some(r) = &args.request {
            r.response_type
        } else {
            ResponseType::TEXT
        };

        let response = if let Some(r) = &args.request {
            if let Some(body) = &r.body {
                handle_json(request, body)?
            } else if let Some(form_data) = &r.form_data {
                handle_form_data(request, form_data)?
            } else {
                request.call().map_err(|e| HttpPluginError::SendRequestError(e.to_string()))?
            }
        } else {
            request.call().map_err(|e| HttpPluginError::SendRequestError(e.to_string()))?
        };
        let parsed_response = parse_response(response, response_type)?;

        Ok(Some(parsed_response))
    }
}

fn handle_form_data(
    request: UreqRequest,
    form_data: &[FormDataEntry],
) -> Result<UreqResponse, PluginError> {
    let mut multipart = Multipart::new();
    for entry in form_data.iter() {
        if entry._type.is_some() {
            if let Some(v) = &entry.value {
                let buf = base64::decode(v).map_err(HttpPluginError::FormValueBase64DecodeError)?;
                let cursor = Cursor::new(buf);
                let file_name = entry.file_name.as_deref();
                multipart.add_stream(entry.name.as_str(), cursor, file_name, None);
            };
        } else if let Some(v) = &entry.value {
            multipart.add_text(entry.name.as_str(), v);
        }
    }
    // Send the request with the multipart/form-data
    let mdata = multipart
        .prepare()
        .map_err(|e| HttpPluginError::MultipartPrepareError(e.to_string()))?;
    let result = request
        .set(
            "Content-Type",
            &format!("multipart/form-data; boundary={}", mdata.boundary()),
        )
        .send(mdata)
        .map_err(|e| HttpPluginError::SendRequestError(e.to_string()))?;

    Ok(result)
}

fn handle_json(request: UreqRequest, body: &str) -> Result<UreqResponse, HttpPluginError> {
    let value = JSON::from_str::<JSON::Value>(body);
    let json = value.map_err(HttpPluginError::JSONParseError)?;

    let result = request
        .send_json(json)
        .map_err(|e| HttpPluginError::SendRequestError(e.to_string()))?;

    Ok(result)
}

#[derive(thiserror::Error, Debug)]
pub enum HttpPluginError {
    #[error("Error sending request: `{0}`")]
    SendRequestError(String),
    #[error("Error parsing JSON: `{0}`")]
    JSONParseError(JSON::Error),
    #[error("Error decoding base64 of form value: `{0}`")]
    FormValueBase64DecodeError(base64::DecodeError),
    #[error("Error preparing multipart data: `{0}`")]
    MultipartPrepareError(String),
}

impl From<HttpPluginError> for PluginError {
    fn from(e: HttpPluginError) -> Self {
        PluginError::InvocationError {
            exception: e.to_string(),
        }
    }
}
