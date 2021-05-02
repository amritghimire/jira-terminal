pub mod request;
use json;
use ureq;
extern crate base64;

/// Call GET API request to JIRA with provided api request.
/// This will act as service to call API.
///
/// # Arguments
/// * api_request - API request structure that will contain information regarding the api call.
///
/// # Example
/// ```
/// let api_request = api::request::ApiRequest {
///  url: url,
///  username: "email@address.com".to_string(),
///  password: "token".to_string(),
///  json: json::object! {},
///  namespace: "namespace".to_string(),
///   version: 3,
///  };
/// assert!(true, api_request(api_request).is_ok());
/// ```
pub fn get(api_request: request::ApiRequest) -> Result<json::JsonValue, ureq::Error> {
    let url = format!(
        "https://{}.atlassian.net/rest/api/{}/{}",
        api_request.namespace, api_request.version, api_request.url
    );
    let user_password = format!("{}:{}", api_request.username, api_request.password);
    let b64 = base64::encode(user_password);
    let authentication = format!("Basic {}", b64);
    let response = ureq::get(&url)
        .set("Authorization", &authentication)
        .call()?
        .into_string()?;
    return Ok(json::parse(&response).unwrap());
}
