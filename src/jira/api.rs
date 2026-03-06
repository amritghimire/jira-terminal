use crate::api;
use crate::api::request::ApiRequest;
use crate::config;
use std::error::Error;

/// Call the GET API Service for provided endpoint.
/// This is a wrapper on existing api service which will add the namespace and authentication from
/// config.
///
/// # Arguments
///
/// * endpoint - Endpoint to call the api. Example: user/search
/// * version - Rest API Version to call. Values will be 2/3 as of now.
fn get_call(endpoint: String, version: u8) -> Result<json::JsonValue, Box<dyn Error>> {
    let api_request = get_api_request(endpoint, json::object! {}, version);
    api::get(api_request)
}

/// Call the POST API Service for provided endpoint.
/// This is a wrapper on existing api service which will add the namespace and authentication from
/// config.
///
/// # Arguments
///
/// * endpoint - Endpoint to call the api. Example: user/search
/// * version - Rest API Version to call. Values will be 2/3 as of now.
pub fn post_call(
    endpoint: String,
    json_value: json::JsonValue,
    version: u8,
) -> Result<String, Box<dyn Error>> {
    let api_request = get_api_request(endpoint, json_value, version);
    api::post(api_request)
}

/// Call the PUT API Service for provided endpoint.
/// This is a wrapper on existing api service which will add the namespace and authentication from
/// config.
///
/// # Arguments
///
/// * endpoint - Endpoint to call the api. Example: user/search
/// * version - Rest API Version to call. Values will be 2/3 as of now.
pub fn put_call(
    endpoint: String,
    json_value: json::JsonValue,
    version: u8,
) -> Result<String, Box<dyn Error>> {
    let api_request = get_api_request(endpoint, json_value, version);
    api::put(api_request)
}

fn get_api_request(endpoint: String, json_value: json::JsonValue, version: u8) -> ApiRequest {
    ApiRequest {
        url: endpoint,
        username: config::get_config("email".to_string()),
        password: config::get_config("token".to_string()),
        json: json_value,
        namespace: config::get_config("namespace".to_string()),
        version,
        auth_mode: config::get_config("auth_mode".to_string()),
    }
}

/// Shortcut for version 2 get_call.
///
/// # Arguments
///
/// * endpoint - Endpoint to call the api. Example: user/search
pub fn get_call_v2(endpoint: String) -> Result<json::JsonValue, Box<dyn Error>> {
    get_call(endpoint, 2)
}

/// Shortcut for version 3 get_call.
///
/// # Arguments
///
/// * endpoint - Endpoint to call the api. Example: user/search
pub fn get_call_v3(endpoint: String) -> Result<json::JsonValue, Box<dyn Error>> {
    get_call(
        endpoint,
        config::get_config("version".to_string())
            .parse::<u8>()
            .unwrap_or(2),
    )
}
