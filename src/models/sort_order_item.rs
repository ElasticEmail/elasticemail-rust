/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// SortOrderItem : Change the ordering of this inbound route for when matching the inbound



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SortOrderItem {
    /// ID of the route to change the order of
    #[serde(rename = "PublicInboundId")]
    pub public_inbound_id: String,
    /// 1 - route will be used first
    #[serde(rename = "SortOrder")]
    pub sort_order: i32,
}

impl SortOrderItem {
    /// Change the ordering of this inbound route for when matching the inbound
    pub fn new(public_inbound_id: String, sort_order: i32) -> SortOrderItem {
        SortOrderItem {
            public_inbound_id,
            sort_order,
        }
    }
}


