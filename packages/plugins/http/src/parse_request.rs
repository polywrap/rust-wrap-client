use crate::{wrap::types::Request, RequestMethod};

pub fn parse_request(url: &str, request: Option<Request>, method: RequestMethod) -> ureq::Request {
    let mut request_builder = match method {
        RequestMethod::GET => ureq::get(url),
        RequestMethod::POST => ureq::post(url),
    };

    if let Some(request) = request {
        if let Some(url_params) = request.url_params {
            for (key, value) in url_params.iter() {
                request_builder = request_builder.query(key, value);
            }
        };

        if let Some(headers) = request.headers {
            for (name, value) in headers.iter() {
                request_builder = request_builder.set(name, value)
            }
        }
    }

    request_builder
}
