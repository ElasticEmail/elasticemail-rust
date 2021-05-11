/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    To start using this API, you will need your Access Token (available <a href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a href=\"https://api.elasticemail.com/public/help\">here</a>.
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// BodyPart : Email body part with user-provided MIME type (text/html, text/plain, etc)



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BodyPart {
    /// Type of the body part
    #[serde(rename = "ContentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<crate::models::BodyContentType>,
    /// Actual content of the body part
    #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Text value of charset encoding for example: iso-8859-1, windows-1251, utf-8, us-ascii, windows-1250 and more…
    #[serde(rename = "Charset", skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
}

impl BodyPart {
    /// Email body part with user-provided MIME type (text/html, text/plain, etc)
    pub fn new() -> BodyPart {
        BodyPart {
            content_type: None,
            content: None,
            charset: None,
        }
    }
}


