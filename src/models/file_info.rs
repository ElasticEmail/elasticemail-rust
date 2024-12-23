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

/// FileInfo : File information
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    /// Name of your file including extension.
    #[serde(rename = "FileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Size of your attachment (in bytes).
    #[serde(rename = "Size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub size: Option<Option<i32>>,
    /// Date of creation in YYYY-MM-DDThh:ii:ss format
    #[serde(rename = "DateAdded", skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    /// Date when the file will be deleted from your Account.
    #[serde(rename = "ExpirationDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<Option<String>>,
    /// Content type of the file.
    #[serde(rename = "ContentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl FileInfo {
    /// File information
    pub fn new() -> FileInfo {
        FileInfo {
            file_name: None,
            size: None,
            date_added: None,
            expiration_date: None,
            content_type: None,
        }
    }
}

