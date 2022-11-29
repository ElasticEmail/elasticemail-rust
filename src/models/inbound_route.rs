/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://app.elasticemail.com/marketing/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InboundRoute {
    #[serde(rename = "PublicId", skip_serializing_if = "Option::is_none")]
    pub public_id: Option<String>,
    /// Name of this route
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "FilterType", skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<crate::models::InboundRouteFilterType>,
    /// Filter of the inbound data
    #[serde(rename = "Filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(rename = "ActionType", skip_serializing_if = "Option::is_none")]
    pub action_type: Option<crate::models::InboundRouteActionType>,
    /// URL address or Email to notify about the inbound
    #[serde(rename = "ActionParameter", skip_serializing_if = "Option::is_none")]
    pub action_parameter: Option<String>,
    /// Place of this route in your routes queue's order
    #[serde(rename = "SortOrder", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
}

impl InboundRoute {
    pub fn new() -> InboundRoute {
        InboundRoute {
            public_id: None,
            name: None,
            filter_type: None,
            filter: None,
            action_type: None,
            action_parameter: None,
            sort_order: None,
        }
    }
}


