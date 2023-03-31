use crate::wrap::types::{Request, Response, ResponseType};
use polywrap_msgpack::extensions::generic_map::GenericMap;
use polywrap_plugin::error::PluginError;
use std::collections::BTreeMap;

pub enum RequestMethod {
    GET,
    POST,
}

pub fn parse_response(
    response: ureq::Response,
    encoding: ResponseType,
) -> Result<Response, PluginError> {
    let headers = response
        .headers_names()
        .iter()
        .map(|header_name| {
            (
                header_name.to_string(),
                response.header(header_name).unwrap().to_string(),
            )
        })
        .collect::<BTreeMap<String, String>>();
    let headers = GenericMap(headers);
    print!("from parse response in http plugin:");
    println!("{:?}", response);
    let status = response.status();
    let status_text = response.status_text().to_string();

    let mut reader = response.into_reader();
    let mut data = vec![];
    reader.read_to_end(&mut data).unwrap();

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

pub fn parse_request(
    url: &str,
    request: Option<Request>,
    method: RequestMethod,
) -> Result<ureq::Request, PluginError> {
    let mut request_builder = match method {
        RequestMethod::GET => ureq::get(url),
        RequestMethod::POST => ureq::post(url),
    };

    if let Some(request) = request {
        if let Some(url_params) = request.url_params {
            for (key, value) in url_params.0.iter() {
                request_builder = request_builder.query(key, value);
            }
        };

        if let Some(headers) = request.headers {
            for (name, value) in headers.0.iter() {
                request_builder = request_builder.set(name, value)
            }
        }
    }

    println!("BUILDER {:?}", request_builder);

    Ok(request_builder)
}
