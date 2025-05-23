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
pub struct ListPayload {
    /// Name of your list.
    #[serde(rename = "ListName")]
    pub list_name: String,
    /// True: Allow unsubscribing from this list. Otherwise, false
    #[serde(rename = "AllowUnsubscribe", skip_serializing_if = "Option::is_none")]
    pub allow_unsubscribe: Option<bool>,
    /// Comma delimited list of existing contact emails that should be added to this list. Leave empty for all contacts
    #[serde(rename = "Emails", skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
}

impl ListPayload {
    pub fn new(list_name: String) -> ListPayload {
        ListPayload {
            list_name,
            allow_unsubscribe: None,
            emails: None,
        }
    }
}

