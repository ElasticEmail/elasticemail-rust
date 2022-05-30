/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// SubaccountEmailCreditsPayload : A change to SubAccount email credits pool, with an additional note.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubaccountEmailCreditsPayload {
    /// Positive or negative value; this will be added or subtracted from Subaccount's current email Credits pool.
    #[serde(rename = "Credits")]
    pub credits: i32,
    /// Note to append to this credits change, for history.
    #[serde(rename = "Notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl SubaccountEmailCreditsPayload {
    /// A change to SubAccount email credits pool, with an additional note.
    pub fn new(credits: i32) -> SubaccountEmailCreditsPayload {
        SubaccountEmailCreditsPayload {
            credits,
            notes: None,
        }
    }
}


