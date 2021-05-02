use json;

pub struct ApiRequest {
    pub url: String,
    pub username: String,
    pub password: String,
    pub json: json::JsonValue,
    pub namespace: String,
    pub version: u8,
}
