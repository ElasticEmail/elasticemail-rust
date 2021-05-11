/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    To start using this API, you will need your Access Token (available <a href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a href=\"https://api.elasticemail.com/public/help\">here</a>.
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Campaign {
    /// Campaign's email content. Provide multiple items to send an A/X Split Campaign
    #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<crate::models::CampaignTemplate>>,
    /// Campaign name
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Campaign status
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::CampaignStatus>,
    /// Recipients this campaign should be sent to
    #[serde(rename = "Recipients", skip_serializing_if = "Option::is_none")]
    pub recipients: Option<crate::models::CampaignRecipient>,
    /// Campaign sending options
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<crate::models::CampaignOptions>,
}

impl Campaign {
    pub fn new() -> Campaign {
        Campaign {
            content: None,
            name: None,
            status: None,
            recipients: None,
            options: None,
        }
    }
}

