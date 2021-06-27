pub mod request;

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
        "https://{}/rest/api/{}/{}",
        api_request.namespace, api_request.version, api_request.url
    );
    let authentication = format!("Basic {}", api_request.password);
    let response = ureq::get(&url)
        .set("Authorization", &authentication)
        .call()?
        .into_string()?;
    Ok(json::parse(&response).unwrap())
}

/// Call POST API request to JIRA with provided api request.
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
///  json: json::object! {"key":"value"},
///  namespace: "namespace".to_string(),
///   version: 3,
///  };
/// assert!(true, api_request(api_request).is_ok());
/// ```
pub fn post(api_request: request::ApiRequest) -> Result<String, ureq::Error> {
    let url = format!(
        "https://{}/rest/api/{}/{}",
        api_request.namespace, api_request.version, api_request.url
    );
    let authentication = format!("Basic {}", api_request.password);
    let response: String = ureq::post(&url)
        .set("Authorization", &authentication)
        .set("Content-Type", "application/json")
        .send_string(&json::stringify(api_request.json))?
        .into_string()?;
    Ok(response)
}

/// Call PUT API request to JIRA with provided api request.
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
///  json: json::object! {"key":"value"},
///  namespace: "namespace".to_string(),
///   version: 3,
///  };
/// assert!(true, api_request(api_request).is_ok());
/// ```
pub fn put(api_request: request::ApiRequest) -> Result<String, ureq::Error> {
    let url = format!(
        "https://{}/rest/api/{}/{}",
        api_request.namespace, api_request.version, api_request.url
    );
    let authentication = format!("Basic {}", api_request.password);
    let response: String = ureq::put(&url)
        .set("Authorization", &authentication)
        .set("Content-Type", "application/json")
        .send_string(&json::stringify(api_request.json))?
        .into_string()?;
    Ok(response)
}
