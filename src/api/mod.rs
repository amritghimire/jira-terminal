use std::error::Error;

pub mod request;

fn handle_response_error_json(
    response: Result<ureq::Response, ureq::Error>,
) -> Result<json::JsonValue, Box<dyn Error>> {
    match response {
        Ok(r) => {
            let response_string = r.into_string()?;
            Ok(json::parse(&response_string).unwrap())
        }
        Err(ureq::Error::Status(code, r)) => {
            eprintln!("JIRA API returned with status code {code}. ");
            let response_string = r.into_string()?;
            match json::parse(&response_string) {
                Ok(j) => {
                    eprintln!("{}", json::stringify_pretty(j, 4));
                }
                Err(_) => {
                    eprintln!("{response_string}");
                }
            }
            Err(Box::new(ureq::Error::Status(
                code,
                ureq::Response::new(code, "API", "API Error").unwrap(),
            )))
        }
        Err(e) => Err(Box::new(e)),
    }
}

fn handle_response_error(
    response: Result<ureq::Response, ureq::Error>,
) -> Result<String, Box<dyn Error>> {
    match response {
        Ok(r) => {
            let response_string = r.into_string()?;
            Ok(response_string)
        }
        Err(ureq::Error::Status(code, r)) => {
            eprintln!("JIRA API returned with status code {code}. ");
            let response_string = r.into_string()?;
            match json::parse(&response_string) {
                Ok(j) => {
                    eprintln!("{}", json::stringify_pretty(j, 4));
                }
                Err(_) => {
                    eprintln!("{response_string}");
                }
            }
            Err(Box::new(ureq::Error::Status(
                code,
                ureq::Response::new(code, "API", "API Error").unwrap(),
            )))
        }
        Err(e) => Err(Box::new(e)),
    }
}

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
pub fn get(api_request: request::ApiRequest) -> Result<json::JsonValue, Box<dyn Error>> {
    let url = format!(
        "https://{}/rest/api/{}/{}",
        api_request.namespace, api_request.version, api_request.url
    );
    let authentication = format!("Basic {}", api_request.password);
    let response = ureq::get(&url).set("Authorization", &authentication).call();
    handle_response_error_json(response)
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
pub fn post(api_request: request::ApiRequest) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://{}/rest/api/{}/{}",
        api_request.namespace, api_request.version, api_request.url
    );
    let authentication = format!("Basic {}", api_request.password);
    let response = ureq::post(&url)
        .set("Authorization", &authentication)
        .set("Content-Type", "application/json")
        .send_string(&json::stringify(api_request.json));
    handle_response_error(response)
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
pub fn put(api_request: request::ApiRequest) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://{}/rest/api/{}/{}",
        api_request.namespace, api_request.version, api_request.url
    );
    let authentication = format!("Basic {}", api_request.password);
    let response = ureq::put(&url)
        .set("Authorization", &authentication)
        .set("Content-Type", "application/json")
        .send_string(&json::stringify(api_request.json));
    handle_response_error(response)
}
