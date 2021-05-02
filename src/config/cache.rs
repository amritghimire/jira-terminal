use json;

#[path = "../api/mod.rs"]
mod api;

pub fn get_username(configuration: &json::JsonValue) -> String {
    let url = format!(
        "user/search?query={}",
        configuration["email"].as_str().unwrap().to_string()
    );
    let api_request = api::request::ApiRequest {
        url: url,
        username: configuration["email"].as_str().unwrap().to_string(),
        password: configuration["token"].as_str().unwrap().to_string(),
        json: json::object! {},
        namespace: configuration["namespace"].as_str().unwrap().to_string(),
        version: 3,
    };
    let response = api::get(api_request).unwrap();
    let account_id = String::from(response[0]["accountId"].as_str().unwrap());
    account_id
}
