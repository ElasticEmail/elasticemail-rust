/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a target=\"_blank\" href=\"https://api.elasticemail.com/public/help\">here</a>.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// SubaccountPayload : New SubAccount payload



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubaccountPayload {
    /// Proper email address.
    #[serde(rename = "Email")]
    pub email: String,
    /// Current password.
    #[serde(rename = "Password")]
    pub password: String,
    /// True, if you want to send activation email to this Account to confirm the creation of a new SubAccount. Otherwise, false (SubAccount will immediately be Active).
    #[serde(rename = "SendActivation", skip_serializing_if = "Option::is_none")]
    pub send_activation: Option<bool>,
    /// SubAccount settings
    #[serde(rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<Box<crate::models::SubaccountSettingsInfoPayload>>,
}

impl SubaccountPayload {
    /// New SubAccount payload
    pub fn new(email: String, password: String) -> SubaccountPayload {
        SubaccountPayload {
            email,
            password,
            send_activation: None,
            settings: None,
        }
    }
}


