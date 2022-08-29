use crate::api;

/// Get the user account id from email provided by user while config creation.
/// For most of the API Call, user email will not be valid due to recent changes in GDPR policies.
/// This function will fetch the unique account id for provided email.
///
/// # Arguments
///
/// * configuration - JSON Object for config stored in config file.
///
/// # Example
///
/// ```
/// let account_id = get_username(&configuration);
/// ```
pub fn get_username(configuration: &json::JsonValue) -> String {
    let url = format!(
        "user/search?query={}",
        configuration["email"].as_str().unwrap()
    );
    let api_request = api::request::ApiRequest {
        url,
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
