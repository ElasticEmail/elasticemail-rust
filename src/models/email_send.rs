/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    To start using this API, you will need your Access Token (available <a href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a href=\"https://api.elasticemail.com/public/help\">here</a>.
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailSend {
    /// ID number of transaction
    #[serde(rename = "TransactionID", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Unique identifier for this email.
    #[serde(rename = "MessageID", skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

impl EmailSend {
    pub fn new() -> EmailSend {
        EmailSend {
            transaction_id: None,
            message_id: None,
        }
    }
}

