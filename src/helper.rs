use std::borrow::BorrowMut;
use worker::{console_log, Env, Headers, Request, Url};

static KV_NAMESPACE: &str = "default";
static KV_ALLOWED_ORIGINS_KEY: &str = "allowed_hostnames";

pub(crate) fn get_origin(request: &Request) -> Result<String, String> {
    // Get origin of the request. This header may not exist for requests that are not cors.
    match request.headers().get("origin") {
        Ok(origin_header) => match origin_header {
            Some(origin) => Ok(origin),
            None => Err("Did not get an origin header from the request.".to_string()),
        },
        Err(error) => Err(format!("Error getting origin header. {error}")),
    }
}

pub(crate) async fn is_allowed(origin: &str, env: &Env) -> Result<bool, (String, u16)> {
    // Our list of allowed origins is looser than the cors spec in that we ignore case and just consider the hostname of the url.
    let hostname = match Url::parse(origin) {
        Ok(url) => {
            if let Some(hostname) = url.host_str() {
                Ok(hostname.to_ascii_lowercase())
            } else {
                Err((format!("Could not get host from '{origin}'"), 400))
            }
        }
        Err(error) => Err((format!("Error parsing '{origin}' as a URL. {error}"), 400)),
    }?;

    // Allowed origins is a comma separated string, so tokenize it and determine if origin is there.
    match env.kv(KV_NAMESPACE) {
        Ok(namespace) => match namespace.get(KV_ALLOWED_ORIGINS_KEY).await {
            Ok(allowed_origins) => {
                if let Some(allowed_origins) = allowed_origins {
                    Ok(allowed_origins
                        .as_string()
                        .to_ascii_lowercase()
                        .split(',')
                        .borrow_mut()
                        .find(|o| o == &hostname)
                        .unwrap_or_default()
                        == hostname)
                } else {
                    Err((format!(
                        "Did not get the key '{KV_ALLOWED_ORIGINS_KEY}' from namespace '{KV_NAMESPACE}'"
                    ), 500))
                }
            }
            Err(error) => {
                Err((format!(
                    "Error getting the key '{KV_ALLOWED_ORIGINS_KEY}' from namespace '{KV_NAMESPACE}'. {error}"
                ), 500))
            }
        },
        Err(error) => {
            Err((format!(
                "Error getting the namespace '{KV_ALLOWED_ORIGINS_KEY}'. {error}",
            ), 500))
        }
    }
}

pub(crate) fn set_header(headers_mut: &mut Headers, key: &str, value: &str) {
    headers_mut
        .set(key, value)
        .map_err(|e| console_log!("Error setting header '{key}':'{value}'. {e}"))
        .ok();
}

pub(crate) fn get_header(headers: &Headers, key: &str) -> String {
    headers
        .get(key)
        .map_err(|e| console_log!("Error getting header '{key}'. {e}"))
        .unwrap_or_default()
        .unwrap_or_default()
}
