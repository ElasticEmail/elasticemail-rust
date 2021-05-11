/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    To start using this API, you will need your Access Token (available <a href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    This is the documentation for REST API. If you’d like to read our legacy documentation regarding Web API v2 click <a href=\"https://api.elasticemail.com/public/help\">here</a>.
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// FileInfo : File information



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileInfo {
    /// Name of your file including extension.
    #[serde(rename = "FileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// Size of your attachment (in bytes).
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Date of creation in YYYY-MM-DDThh:ii:ss format
    #[serde(rename = "DateAdded", skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    /// Date when the file will be deleted from your Account.
    #[serde(rename = "ExpirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
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

