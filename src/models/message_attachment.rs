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
pub struct MessageAttachment {
    /// File's content as byte array (or a Base64 string)
    #[serde(rename = "BinaryContent")]
    pub binary_content: String,
    /// Display name of the file
    #[serde(rename = "Name")]
    pub name: String,
    /// MIME content type
    #[serde(rename = "ContentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Size of your attachment (in bytes).
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

impl MessageAttachment {
    pub fn new(binary_content: String, name: String) -> MessageAttachment {
        MessageAttachment {
            binary_content,
            name,
            content_type: None,
            size: None,
        }
    }
}


