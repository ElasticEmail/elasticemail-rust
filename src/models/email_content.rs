/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    To start using this API, you will need your Access Token (available <a href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a href=\"https://api.elasticemail.com/public/help\">here</a>.
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// EmailContent : Proper e-mail content



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EmailContent {
    /// List of e-mail body parts, with user-provided MIME types (text/html, text/plain etc)
    #[serde(rename = "Body", skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<crate::models::BodyPart>>,
    /// A key-value collection of custom merge fields, shared between recipients. Should be used in e-mail body like so: {firstname}, {lastname} etc.
    #[serde(rename = "Merge", skip_serializing_if = "Option::is_none")]
    pub merge: Option<::std::collections::HashMap<String, String>>,
    /// Attachments provided by sending binary data
    #[serde(rename = "Attachments", skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<crate::models::MessageAttachment>>,
    /// A key-value collection of custom e-mail headers.
    #[serde(rename = "Headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    /// Postback header.
    #[serde(rename = "Postback", skip_serializing_if = "Option::is_none")]
    pub postback: Option<String>,
    /// Your e-mail with an optional name (e.g.: John Doe <email@domain.com>)
    #[serde(rename = "From", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
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
    /// Utm marketing data to be attached to every link in this e-mail.
    #[serde(rename = "Utm", skip_serializing_if = "Option::is_none")]
    pub utm: Option<crate::models::Utm>,
}

impl EmailContent {
    /// Proper e-mail content
    pub fn new() -> EmailContent {
        EmailContent {
            body: None,
            merge: None,
            attachments: None,
            headers: None,
            postback: None,
            from: None,
            reply_to: None,
            subject: None,
            template_name: None,
            attach_files: None,
            utm: None,
        }
    }
}


