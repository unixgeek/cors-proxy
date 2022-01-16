use std::borrow::BorrowMut;
use worker::{console_log, Env, Headers, Request, Url};

static KV_NAMESPACE: &str = "default";
static KV_ALLOWED_ORIGINS_KEY: &str = "allowed_hostnames";

pub(crate) fn get_origin(request: &Request) -> String {
    // Get origin of the request. This header may not exist for requests that are not cors.
    match request.headers().get("origin") {
        Ok(origin_header) => match origin_header {
            Some(origin) => origin,
            None => {
                console_log!("Did not get an origin header from the request.");
                String::default()
            }
        },
        Err(error) => {
            console_log!("Error getting origin header. {}", error);
            String::default()
        }
    }
}

pub(crate) async fn is_allowed(origin: &str, env: Env) -> bool {
    // Our list of allowed origins is looser than the cors spec in that we ignore case and just consider the hostname of the url.
    let hostname = match Url::parse(origin) {
        Ok(url) => {
            if let Some(hostname) = url.host_str() {
                hostname.to_ascii_lowercase()
            } else {
                String::default()
            }
        }
        Err(error) => {
            console_log!("Error parsing {} as a URL. {}", origin, error);
            String::default()
        }
    };

    // Allowed origins is a comma separated string, so tokenize it and determine if origin is there.
    match env.kv(KV_NAMESPACE) {
        Ok(namespace) => match namespace.get(KV_ALLOWED_ORIGINS_KEY).await {
            Ok(allowed_origins) => {
                if let Some(allowed_origins) = allowed_origins {
                    allowed_origins
                        .as_string()
                        .to_ascii_lowercase()
                        .split(',')
                        .borrow_mut()
                        .find(|o| o == &hostname)
                        .unwrap_or_default()
                        == hostname
                } else {
                    console_log!(
                        "Did not get the key {} from namespace {}",
                        KV_ALLOWED_ORIGINS_KEY,
                        KV_NAMESPACE
                    );
                    false
                }
            }
            Err(error) => {
                console_log!(
                    "Error getting the key {} from namespace {}. {}",
                    KV_ALLOWED_ORIGINS_KEY,
                    KV_NAMESPACE,
                    error
                );
                false
            }
        },
        Err(error) => {
            console_log!(
                "Error getting the namespace {}. {}",
                KV_ALLOWED_ORIGINS_KEY,
                error
            );
            false
        }
    }
}

pub(crate) fn set_header(headers_mut: &mut Headers, key: &str, value: &str) {
    headers_mut
        .set(key, value)
        .map_err(|e| console_log!("Error setting header {}:{}. {}", key, value, e))
        .ok();
}

pub(crate) fn get_header(headers: &Headers, key: &str) -> String {
    headers
        .get(key)
        .map_err(|e| console_log!("Error getting header {}. {}", key, e))
        .unwrap_or_default()
        .unwrap_or_default()
}
