use worker::*;

mod helper;
mod proxy;
mod util;

#[event(fetch)]
pub async fn main(request: Request, env: Env, _ctx: Context) -> Result<Response> {
    util::set_panic_hook();
    util::log_request(&request);

    let origin = match helper::get_origin(&request) {
        Ok(o) => o,
        Err(error) => {
            return proxy::create_error_response(&error, 400, "*");
        }
    };

    // Check if origin is allowed.
    match helper::is_allowed(&origin, &env).await {
        Ok(is_allowed) => {
            if !is_allowed {
                return proxy::create_error_response(
                    &format!("Origin '{origin}' is not allowed."),
                    403,
                    &origin,
                );
            }
        }
        Err(error) => {
            // Error could be client or code related and we need to set wildcard to *hopefully* get the response back to the client.
            return proxy::create_error_response(&error.0, error.1, "*");
        }
    }

    if request.method() == Method::Options {
        // Intercept OPTIONS (Preflight) request and don't proxy it.
        proxy::create_options_response(request, &origin)
    } else if let Some(target_url) = request
        .url()?
        .query_pairs()
        .find_map(|(key, value)| match key.as_ref() {
            "url" => Some(value.to_string()),
            _ => None,
        })
    {
        // If the url parameter is provided, attempt to proxy the request.
        let proxy_request = proxy::copy_request(request, &target_url).await;
        if proxy_request.is_err() {
            console_log!(
                "There was an error copying the request. {}",
                proxy_request.err().unwrap()
            );
            return proxy::create_error_response("Could not copy request", 500, &origin);
        }

        let fetch = Fetch::Request(proxy_request.unwrap());
        match fetch.send().await {
            Ok(proxy_response) => {
                let response = proxy::copy_response(proxy_response, &origin).await;
                if response.is_err() {
                    console_log!(
                        "There was an error copying the response. {}",
                        response.err().unwrap()
                    );
                    return proxy::create_error_response("Could not copy response", 500, &origin);
                }
                response
            }
            Err(error) => {
                console_log!("There was an error fetching the request. {}", error);
                proxy::create_error_response(&error.to_string(), 500, &origin)
            }
        }
    } else {
        // No url was provided.
        proxy::create_error_response("Missing required url parameter.", 400, &origin)
    }
}
