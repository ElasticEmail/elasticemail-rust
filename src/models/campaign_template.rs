/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://app.elasticemail.com/marketing/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// CampaignTemplate : Content of a Campaign



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CampaignTemplate {
    /// Name of your custom IP Pool to be used in the sending process
    #[serde(rename = "Poolname", skip_serializing_if = "Option::is_none")]
    pub poolname: Option<String>,
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
    #[serde(rename = "Utm", skip_serializing_if = "Option::is_none")]
    pub utm: Option<Box<crate::models::Utm>>,
}

impl CampaignTemplate {
    /// Content of a Campaign
    pub fn new() -> CampaignTemplate {
        CampaignTemplate {
            poolname: None,
            from: None,
            reply_to: None,
            subject: None,
            template_name: None,
            attach_files: None,
            utm: None,
        }
    }
}


