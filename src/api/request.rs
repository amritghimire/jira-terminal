/// Structure to hold api request.
pub struct ApiRequest {
    /// URL to hold make the request. Start without forward slash.
    /// Example: user/search
    pub url: String,
    /// Username/email for login purpose.
    pub username: String,
    /// Password/Token for user.
    /// User can go to `https://id.atlassian.com/manage-profile/security/api-tokens` for tokens.
    pub password: String,
    /// JSON to send for POST, PATCH and PUT request.
    pub json: json::JsonValue,
    /// Namespace of JIRA. For a jira dashboard at `https://example.atlassian.net`, *example* is
    /// namespace.
    pub namespace: String,
    /// Rest API Version to use.
    pub version: u8,
}
