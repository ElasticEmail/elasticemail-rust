/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    To start using this API, you will need your Access Token (available <a href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a href=\"https://api.elasticemail.com/public/help\">here</a>.
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// EventType : Type of event

/// Type of event
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "Submission")]
    Submission,
    #[serde(rename = "FailedAttempt")]
    FailedAttempt,
    #[serde(rename = "Bounce")]
    Bounce,
    #[serde(rename = "Sent")]
    Sent,
    #[serde(rename = "Open")]
    Open,
    #[serde(rename = "Click")]
    Click,
    #[serde(rename = "Unsubscribe")]
    Unsubscribe,
    #[serde(rename = "Complaint")]
    Complaint,

}




