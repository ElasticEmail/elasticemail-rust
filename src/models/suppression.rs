/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    To start using this API, you will need your Access Token (available <a href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a href=\"https://api.elasticemail.com/public/help\">here</a>.
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// Suppression : Suppression - Email returning Hard Bounces



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Suppression {
    /// Proper email address.
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// RFC error message
    #[serde(rename = "FriendlyErrorMessage", skip_serializing_if = "Option::is_none")]
    pub friendly_error_message: Option<String>,
    /// SMTP Error code
    #[serde(rename = "ErrorCode", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i32>,
    /// Last change date
    #[serde(rename = "DateUpdated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
}

impl Suppression {
    /// Suppression - Email returning Hard Bounces
    pub fn new() -> Suppression {
        Suppression {
            email: None,
            friendly_error_message: None,
            error_code: None,
            date_updated: None,
        }
    }
}

