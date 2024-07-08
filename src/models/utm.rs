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
use serde::{Deserialize, Serialize};

/// Utm : Utm marketing data to be attached to every link in this e-mail.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Utm {
    /// utmsource value
    #[serde(rename = "Source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// utmmedium value
    #[serde(rename = "Medium", skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    /// utmcampaign value
    #[serde(rename = "Campaign", skip_serializing_if = "Option::is_none")]
    pub campaign: Option<String>,
    /// utmcontent value
    #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl Utm {
    /// Utm marketing data to be attached to every link in this e-mail.
    pub fn new() -> Utm {
        Utm {
            source: None,
            medium: None,
            campaign: None,
            content: None,
        }
    }
}

