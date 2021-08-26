/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a target=\"_blank\" href=\"https://api.elasticemail.com/public/help\">here</a>.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// RecipientEvent : Detailed information about message recipient



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecipientEvent {
    /// ID number of transaction
    #[serde(rename = "TransactionID", skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// ID number of selected message.
    #[serde(rename = "MsgID", skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<String>,
    /// Default From: email address.
    #[serde(rename = "FromEmail", skip_serializing_if = "Option::is_none")]
    pub from_email: Option<String>,
    /// Ending date for search in YYYY-MM-DDThh:mm:ss format.
    #[serde(rename = "To", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Default subject of email.
    #[serde(rename = "Subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Type of an Event
    #[serde(rename = "EventType", skip_serializing_if = "Option::is_none")]
    pub event_type: Option<Box<crate::models::EventType>>,
    /// Creation date
    #[serde(rename = "EventDate", skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    /// Name of selected channel.
    #[serde(rename = "ChannelName", skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// Message category
    #[serde(rename = "MessageCategory", skip_serializing_if = "Option::is_none")]
    pub message_category: Option<Box<crate::models::MessageCategory>>,
    /// Date of next try
    #[serde(rename = "NextTryOn", skip_serializing_if = "Option::is_none")]
    pub next_try_on: Option<String>,
    /// Content of message, HTML encoded
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// IP which this email was sent through
    #[serde(rename = "IPAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Name of an IP pool this email was sent through
    #[serde(rename = "PoolName", skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
}

impl RecipientEvent {
    /// Detailed information about message recipient
    pub fn new() -> RecipientEvent {
        RecipientEvent {
            transaction_id: None,
            msg_id: None,
            from_email: None,
            to: None,
            subject: None,
            event_type: None,
            event_date: None,
            channel_name: None,
            message_category: None,
            next_try_on: None,
            message: None,
            ip_address: None,
            pool_name: None,
        }
    }
}


