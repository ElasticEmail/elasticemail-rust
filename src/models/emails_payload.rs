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

/// EmailsPayload : Provide either rule or a list of emails, not both.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailsPayload {
    /// SQL-like rule. Sending 'All' as a value loads all resources of the given type. Help for building a segment rule can be found here: https://help.elasticemail.com/en/articles/5162182-segment-rules
    #[serde(rename = "Rule", skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    /// Comma delimited list of contact emails
    #[serde(rename = "Emails", skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
}

impl EmailsPayload {
    /// Provide either rule or a list of emails, not both.
    pub fn new() -> EmailsPayload {
        EmailsPayload {
            rule: None,
            emails: None,
        }
    }
}

