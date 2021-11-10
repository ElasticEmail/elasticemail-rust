/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a target=\"_blank\" href=\"https://api.elasticemail.com/public/help\">here</a>.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Campaign {
    /// Campaign's email content. Provide multiple items to send an A/X Split Campaign
    #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<crate::models::CampaignTemplate>>,
    /// Campaign name
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::models::CampaignStatus>,
    #[serde(rename = "Recipients")]
    pub recipients: Box<crate::models::CampaignRecipient>,
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<crate::models::CampaignOptions>>,
}

impl Campaign {
    pub fn new(name: String, recipients: crate::models::CampaignRecipient) -> Campaign {
        Campaign {
            content: None,
            name,
            status: None,
            recipients: Box::new(recipients),
            options: None,
        }
    }
}


