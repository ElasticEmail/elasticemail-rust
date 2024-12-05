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

/// NewApiKey : Newly generated ApiKey with Token
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewApiKey {
    /// Unique token to be used in the system
    #[serde(rename = "Token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// Access level or permission to be assigned to this ApiKey.
    #[serde(rename = "AccessLevel", skip_serializing_if = "Option::is_none")]
    pub access_level: Option<Vec<models::AccessLevel>>,
    /// Name of the ApiKey.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Date this ApiKey was created.
    #[serde(rename = "DateCreated", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Date this ApiKey was last used.
    #[serde(rename = "LastUse", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_use: Option<Option<String>>,
    /// Date this ApiKey expires.
    #[serde(rename = "Expires", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expires: Option<Option<String>>,
    /// Which IPs can use this ApiKey
    #[serde(rename = "RestrictAccessToIPRange", skip_serializing_if = "Option::is_none")]
    pub restrict_access_to_ip_range: Option<Vec<String>>,
}

impl NewApiKey {
    /// Newly generated ApiKey with Token
    pub fn new() -> NewApiKey {
        NewApiKey {
            token: None,
            access_level: None,
            name: None,
            date_created: None,
            last_use: None,
            expires: None,
            restrict_access_to_ip_range: None,
        }
    }
}

