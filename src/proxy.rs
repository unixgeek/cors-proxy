use crate::{get_header, set_header};
use worker::js_sys::Uint8Array;
use worker::{console_log, Headers, Request, RequestInit, Response, Result};

static IGNORE_HEADERS: [&str; 14] = [
    "cdn-loop",
    "cf-connecting-ip",
    "cf-ipcountry",
    "cf-ray",
    "cf-visitor",
    "cf-worker",
    "origin",
    "referer",
    "true-client-ip",
    "x-forwarded-by",
    "x-forwarded-for",
    "x-forwarded-host",
    "x-forwarded-proto",
    "x-real-ip",
];

pub(crate) fn create_options_response(request: Request, origin: &str) -> Result<Response> {
    let mut options_response = Response::empty()?;
    let headers = options_response.headers_mut();

    // Parrot back request headers with allow headers.
    set_header(
        headers,
        "access-control-allow-methods",
        &get_header(request.headers(), "access-control-request-method"),
    );
    set_header(
        headers,
        "access-control-allow-headers",
        &get_header(request.headers(), "access-control-request-headers"),
    );

    // Set allowed origin to incoming origin.
    set_header(headers, "access-control-allow-origin", origin);
    set_header(headers, "access-control-allow-credentials", "true");
    set_header(headers, "access-control-max-age", "3600");

    Ok(options_response.with_status(204))
}

pub(crate) fn create_error_response(message: &str, status: u16, origin: &str) -> Result<Response> {
    console_log!("{message}");
    let mut response = Response::error(message, status)?;
    set_header(
        response.headers_mut(),
        "access-control-allow-origin",
        origin,
    );
    set_header(response.headers_mut(), "content-type", "text/plain");
    Ok(response)
}

pub(crate) async fn copy_request(mut request: Request, target_url: &str) -> Result<Request> {
    let mut request_copy_init = RequestInit::new();

    // Copy method.
    request_copy_init.with_method(request.method());

    // Copy headers.
    let mut request_copy_headers = Headers::new();
    request.headers().entries().for_each(|(key, value)| {
        if !IGNORE_HEADERS.contains(&key.as_str()) {
            set_header(&mut request_copy_headers, &key, &value);
        }
    });
    request_copy_init.with_headers(request_copy_headers);

    // Copy body.
    let body = request.bytes().await?;
    if !body.is_empty() {
        request_copy_init.with_body(Some(Uint8Array::from(body.as_slice()).into()));
    }

    // Create request.
    Request::new_with_init(target_url, &request_copy_init)
}

pub(crate) async fn copy_response(mut response: Response, origin: &str) -> Result<Response> {
    // Initialize from body, if it exists, empty otherwise.
    let body = response.bytes().await?;
    let mut response_copy = match body.is_empty() {
        true => Response::empty()?,
        false => Response::from_bytes(body)?,
    };

    // Copy headers.
    let headers_mut = response_copy.headers_mut();
    response.headers().entries().for_each(|(key, value)| {
        set_header(headers_mut, &key, &value);
    });

    set_header(headers_mut, "access-control-allow-origin", origin);

    Ok(response_copy.with_status(response.status_code()))
}
