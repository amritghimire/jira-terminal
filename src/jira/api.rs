use crate::api;
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
    let api_request = api::request::ApiRequest {
        url: endpoint,
        username: config::get_config("email".to_string()),
        password: config::get_config("token".to_string()),
        json: json::object! {},
        namespace: config::get_config("namespace".to_string()),
        version,
    };
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
    let api_request = api::request::ApiRequest {
        url: endpoint,
        username: config::get_config("email".to_string()),
        password: config::get_config("token".to_string()),
        json: json_value,
        namespace: config::get_config("namespace".to_string()),
        version,
    };
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
    let api_request = api::request::ApiRequest {
        url: endpoint,
        username: config::get_config("email".to_string()),
        password: config::get_config("token".to_string()),
        json: json_value,
        namespace: config::get_config("namespace".to_string()),
        version,
    };
    api::put(api_request)
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
    get_call(endpoint, 3)
}
