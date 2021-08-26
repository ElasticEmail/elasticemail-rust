/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a target=\"_blank\" href=\"https://api.elasticemail.com/public/help\">here</a>.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// EmailView : Email details formatted in json



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailView {
    /// Body (HTML, otherwise plain text) of email
    #[serde(rename = "Body", skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Default subject of email.
    #[serde(rename = "Subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// From email address
    #[serde(rename = "From", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}

impl EmailView {
    /// Email details formatted in json
    pub fn new() -> EmailView {
        EmailView {
            body: None,
            subject: None,
            from: None,
        }
    }
}


