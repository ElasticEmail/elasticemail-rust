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

/// LogStatusSummary : Summary of log status
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LogStatusSummary {
    /// Number of recipients
    #[serde(rename = "Recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<i64>,
    /// Number of emails
    #[serde(rename = "EmailTotal", skip_serializing_if = "Option::is_none")]
    pub email_total: Option<i64>,
    /// Number of SMS
    #[serde(rename = "SmsTotal", skip_serializing_if = "Option::is_none")]
    pub sms_total: Option<i64>,
    /// Number of delivered messages
    #[serde(rename = "Delivered", skip_serializing_if = "Option::is_none")]
    pub delivered: Option<i64>,
    /// Number of bounced messages
    #[serde(rename = "Bounced", skip_serializing_if = "Option::is_none")]
    pub bounced: Option<i64>,
    /// Number of messages in progress
    #[serde(rename = "InProgress", skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<i64>,
    /// Number of opened messages
    #[serde(rename = "Opened", skip_serializing_if = "Option::is_none")]
    pub opened: Option<i64>,
    /// Number of clicked messages
    #[serde(rename = "Clicked", skip_serializing_if = "Option::is_none")]
    pub clicked: Option<i64>,
    /// Number of unsubscribed messages
    #[serde(rename = "Unsubscribed", skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<i64>,
    /// Number of complaint messages
    #[serde(rename = "Complaints", skip_serializing_if = "Option::is_none")]
    pub complaints: Option<i64>,
    /// Number of inbound messages
    #[serde(rename = "Inbound", skip_serializing_if = "Option::is_none")]
    pub inbound: Option<i64>,
    /// Number of manually cancelled messages
    #[serde(rename = "ManualCancel", skip_serializing_if = "Option::is_none")]
    pub manual_cancel: Option<i64>,
    /// Number of messages flagged with 'Not Delivered'
    #[serde(rename = "NotDelivered", skip_serializing_if = "Option::is_none")]
    pub not_delivered: Option<i64>,
}

impl LogStatusSummary {
    /// Summary of log status
    pub fn new() -> LogStatusSummary {
        LogStatusSummary {
            recipients: None,
            email_total: None,
            sms_total: None,
            delivered: None,
            bounced: None,
            in_progress: None,
            opened: None,
            clicked: None,
            unsubscribed: None,
            complaints: None,
            inbound: None,
            manual_cancel: None,
            not_delivered: None,
        }
    }
}

