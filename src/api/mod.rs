pub mod request;
use json;
use ureq;
extern crate base64;

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
