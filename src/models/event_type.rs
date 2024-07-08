/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://app.elasticemail.com/marketing/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

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

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Submission => write!(f, "Submission"),
            Self::FailedAttempt => write!(f, "FailedAttempt"),
            Self::Bounce => write!(f, "Bounce"),
            Self::Sent => write!(f, "Sent"),
            Self::Open => write!(f, "Open"),
            Self::Click => write!(f, "Click"),
            Self::Unsubscribe => write!(f, "Unsubscribe"),
            Self::Complaint => write!(f, "Complaint"),
        }
    }
}

impl Default for EventType {
    fn default() -> EventType {
        Self::Submission
    }
}

