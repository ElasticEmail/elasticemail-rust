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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailValidationResult {
    /// Local part of an email
    #[serde(rename = "Account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Name of selected domain.
    #[serde(rename = "Domain", skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// Full email address that was verified
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Suggested spelling if a possible mistake was found
    #[serde(rename = "SuggestedSpelling", skip_serializing_if = "Option::is_none")]
    pub suggested_spelling: Option<String>,
    /// Does the email have a temporary domain
    #[serde(rename = "Disposable", skip_serializing_if = "Option::is_none")]
    pub disposable: Option<bool>,
    /// Is an email a role email (e.g. info@, noreply@ etc.)
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<bool>,
    /// All detected issues
    #[serde(rename = "Reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Added date
    #[serde(rename = "DateAdded", skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    #[serde(rename = "Result", skip_serializing_if = "Option::is_none")]
    pub result: Option<models::EmailValidationStatus>,
    /// Predicted score
    #[serde(rename = "PredictedScore", skip_serializing_if = "Option::is_none")]
    pub predicted_score: Option<f64>,
    #[serde(rename = "PredictedStatus", skip_serializing_if = "Option::is_none")]
    pub predicted_status: Option<models::EmailPredictedValidationStatus>,
}

impl EmailValidationResult {
    pub fn new() -> EmailValidationResult {
        EmailValidationResult {
            account: None,
            domain: None,
            email: None,
            suggested_spelling: None,
            disposable: None,
            role: None,
            reason: None,
            date_added: None,
            result: None,
            predicted_score: None,
            predicted_status: None,
        }
    }
}

