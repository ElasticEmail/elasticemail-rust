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

/// BodyPart : Email body part with user-provided MIME type (text/html, text/plain, etc)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BodyPart {
    #[serde(rename = "ContentType")]
    pub content_type: models::BodyContentType,
    /// Actual content of the body part
    #[serde(rename = "Content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Text value of charset encoding for example: iso-8859-1, windows-1251, utf-8, us-ascii, windows-1250 and more...
    #[serde(rename = "Charset", skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
}

impl BodyPart {
    /// Email body part with user-provided MIME type (text/html, text/plain, etc)
    pub fn new(content_type: models::BodyContentType) -> BodyPart {
        BodyPart {
            content_type,
            content: None,
            charset: None,
        }
    }
}

