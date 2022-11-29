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
pub struct ContactActivity {
    /// Total emails sent.
    #[serde(rename = "TotalSent", skip_serializing_if = "Option::is_none")]
    pub total_sent: Option<i32>,
    /// Total emails opened.
    #[serde(rename = "TotalOpened", skip_serializing_if = "Option::is_none")]
    pub total_opened: Option<i32>,
    /// Total emails clicked
    #[serde(rename = "TotalClicked", skip_serializing_if = "Option::is_none")]
    pub total_clicked: Option<i32>,
    /// Total emails failed.
    #[serde(rename = "TotalFailed", skip_serializing_if = "Option::is_none")]
    pub total_failed: Option<i32>,
    /// Last date when an email was sent to this contact
    #[serde(rename = "LastSent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_sent: Option<Option<String>>,
    /// Date this contact last opened an email
    #[serde(rename = "LastOpened", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_opened: Option<Option<String>>,
    /// Date this contact last clicked an email
    #[serde(rename = "LastClicked", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_clicked: Option<Option<String>>,
    /// Last date when an email sent to this contact bounced
    #[serde(rename = "LastFailed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_failed: Option<Option<String>>,
    /// IP from which this contact opened or clicked their email last time
    #[serde(rename = "LastIP", skip_serializing_if = "Option::is_none")]
    pub last_ip: Option<String>,
    /// Last RFC Error code if any occurred
    #[serde(rename = "ErrorCode", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<Option<i32>>,
    /// Last RFC error message if any occurred
    #[serde(rename = "FriendlyErrorMessage", skip_serializing_if = "Option::is_none")]
    pub friendly_error_message: Option<String>,
}

impl ContactActivity {
    pub fn new() -> ContactActivity {
        ContactActivity {
            total_sent: None,
            total_opened: None,
            total_clicked: None,
            total_failed: None,
            last_sent: None,
            last_opened: None,
            last_clicked: None,
            last_failed: None,
            last_ip: None,
            error_code: None,
            friendly_error_message: None,
        }
    }
}


