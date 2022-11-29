/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://app.elasticemail.com/marketing/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// SubaccountEmailSettingsPayload : Settings related to sending emails



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SubaccountEmailSettingsPayload {
    /// True, if Account needs credits to send emails. Otherwise, false
    #[serde(rename = "RequiresEmailCredits", skip_serializing_if = "Option::is_none")]
    pub requires_email_credits: Option<bool>,
    /// Maximum size of email including attachments in MB's
    #[serde(rename = "EmailSizeLimit", skip_serializing_if = "Option::is_none")]
    pub email_size_limit: Option<i32>,
    /// Amount of emails Account can send daily
    #[serde(rename = "DailySendLimit", skip_serializing_if = "Option::is_none")]
    pub daily_send_limit: Option<i32>,
    /// Maximum number of contacts the Account can have. 0 means that parent account's limit is used.
    #[serde(rename = "MaxContacts", skip_serializing_if = "Option::is_none")]
    pub max_contacts: Option<i32>,
    /// Can the SubAccount purchase Private IP for themselves
    #[serde(rename = "EnablePrivateIPPurchase", skip_serializing_if = "Option::is_none")]
    pub enable_private_ip_purchase: Option<bool>,
    /// Name of your custom IP Pool to be used in the sending process
    #[serde(rename = "PoolName", skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
    #[serde(rename = "ValidSenderDomainOnly", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub valid_sender_domain_only: Option<Option<bool>>,
}

impl SubaccountEmailSettingsPayload {
    /// Settings related to sending emails
    pub fn new() -> SubaccountEmailSettingsPayload {
        SubaccountEmailSettingsPayload {
            requires_email_credits: None,
            email_size_limit: None,
            daily_send_limit: None,
            max_contacts: None,
            enable_private_ip_purchase: None,
            pool_name: None,
            valid_sender_domain_only: None,
        }
    }
}


