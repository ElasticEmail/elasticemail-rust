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

/// EmailContent : Proper e-mail content
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailContent {
    /// List of e-mail body parts, with user-provided MIME types (text/html, text/plain etc)
    #[serde(rename = "Body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<models::BodyPart>>,
    /// A key-value collection of custom merge fields, shared between recipients. Should be used in e-mail body like so: {firstname}, {lastname} etc.
    #[serde(rename = "Merge", skip_serializing_if = "Option::is_none")]
    pub merge: Option<std::collections::HashMap<String, String>>,
    /// Attachments provided by sending binary data
    #[serde(rename = "Attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<models::MessageAttachment>>,
    /// A key-value collection of custom e-mail headers.
    #[serde(rename = "Headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    /// Postback header.
    #[serde(rename = "Postback", skip_serializing_if = "Option::is_none")]
    pub postback: Option<String>,
    /// E-mail with an optional name to be used as the envelope from address (e.g.: John Doe <email@domain.com>)
    #[serde(rename = "EnvelopeFrom", skip_serializing_if = "Option::is_none")]
    pub envelope_from: Option<String>,
    /// Your e-mail with an optional name (e.g.: John Doe <email@domain.com>)
    #[serde(rename = "From")]
    pub from: String,
    /// To what address should the recipients reply to (e.g. John Doe <email@domain.com>)
    #[serde(rename = "ReplyTo", skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// Default subject of email.
    #[serde(rename = "Subject", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// Name of template.
    #[serde(rename = "TemplateName", skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    /// Names of previously uploaded files that should be sent as downloadable attachments
    #[serde(rename = "AttachFiles", skip_serializing_if = "Option::is_none")]
    pub attach_files: Option<Vec<String>>,
    #[serde(rename = "Utm", skip_serializing_if = "Option::is_none")]
    pub utm: Option<Box<models::Utm>>,
}

impl EmailContent {
    /// Proper e-mail content
    pub fn new(from: String) -> EmailContent {
        EmailContent {
            body: None,
            merge: None,
            attachments: None,
            headers: None,
            postback: None,
            envelope_from: None,
            from,
            reply_to: None,
            subject: None,
            template_name: None,
            attach_files: None,
            utm: None,
        }
    }
}

