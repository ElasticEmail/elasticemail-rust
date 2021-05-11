/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    To start using this API, you will need your Access Token (available <a href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a href=\"https://api.elasticemail.com/public/help\">here</a>.
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// SubAccountInfo : Detailed information about SubAccount.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubAccountInfo {
    /// Public key for limited access to your Account such as contact/add so you can use it safely on public websites.
    #[serde(rename = "PublicAccountID", skip_serializing_if = "Option::is_none")]
    pub public_account_id: Option<String>,
    /// Proper email address.
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// SubAccount settings
    #[serde(rename = "Settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<crate::models::SubaccountSettingsInfo>,
    /// Date of last activity on Account
    #[serde(rename = "LastActivity", skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<String>,
    /// Amount of email credits
    #[serde(rename = "EmailCredits", skip_serializing_if = "Option::is_none")]
    pub email_credits: Option<i32>,
    /// Amount of emails sent from this Account
    #[serde(rename = "TotalEmailsSent", skip_serializing_if = "Option::is_none")]
    pub total_emails_sent: Option<i64>,
    /// Numeric reputation
    #[serde(rename = "Reputation", skip_serializing_if = "Option::is_none")]
    pub reputation: Option<f64>,
    /// Account's current status.
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::AccountStatusEnum>,
    /// How many contacts this SubAccount has stored
    #[serde(rename = "ContactsCount", skip_serializing_if = "Option::is_none")]
    pub contacts_count: Option<i32>,
}

impl SubAccountInfo {
    /// Detailed information about SubAccount.
    pub fn new() -> SubAccountInfo {
        SubAccountInfo {
            public_account_id: None,
            email: None,
            settings: None,
            last_activity: None,
            email_credits: None,
            total_emails_sent: None,
            reputation: None,
            status: None,
            contacts_count: None,
        }
    }
}


