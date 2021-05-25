/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a target=\"_blank\" href=\"https://api.elasticemail.com/public/help\">here</a>.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListPayload {
    /// Name of your list.
    #[serde(rename = "ListName", skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    /// True: Allow unsubscribing from this list. Otherwise, false
    #[serde(rename = "AllowUnsubscribe", skip_serializing_if = "Option::is_none")]
    pub allow_unsubscribe: Option<bool>,
    /// Comma delimited list of existing contact emails that should be added to this list
    #[serde(rename = "Emails", skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
}

impl ListPayload {
    pub fn new() -> ListPayload {
        ListPayload {
            list_name: None,
            allow_unsubscribe: None,
            emails: None,
        }
    }
}


