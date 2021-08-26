/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a target=\"_blank\" href=\"https://api.elasticemail.com/public/help\">here</a>.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// ApiKey : ApiKey info



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiKey {
    /// Access level or permission to be assigned to this ApiKey.
    #[serde(rename = "AccessLevel", skip_serializing_if = "Option::is_none")]
    pub access_level: Option<Vec<crate::models::AccessLevel>>,
    /// Name of the ApiKey.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Date this ApiKey was created.
    #[serde(rename = "DateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Date this ApiKey was last used.
    #[serde(rename = "LastUse", skip_serializing_if = "Option::is_none")]
    pub last_use: Option<String>,
    /// Date this ApiKey expires.
    #[serde(rename = "Expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// Which IPs can use this ApiKey
    #[serde(rename = "RestrictAccessToIPRange", skip_serializing_if = "Option::is_none")]
    pub restrict_access_to_ip_range: Option<Vec<String>>,
}

impl ApiKey {
    /// ApiKey info
    pub fn new() -> ApiKey {
        ApiKey {
            access_level: None,
            name: None,
            date_created: None,
            last_use: None,
            expires: None,
            restrict_access_to_ip_range: None,
        }
    }
}


