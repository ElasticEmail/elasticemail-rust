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

/// VerificationFileResult : Simple verification file result info
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VerificationFileResult {
    /// Identifier of this verification result
    #[serde(rename = "VerificationID", skip_serializing_if = "Option::is_none")]
    pub verification_id: Option<String>,
    /// Origin file name
    #[serde(rename = "Filename", skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "VerificationStatus", skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<models::VerificationStatus>,
    #[serde(rename = "FileUploadResult", skip_serializing_if = "Option::is_none")]
    pub file_upload_result: Option<Box<models::FileUploadResult>>,
    /// Date of creation in YYYY-MM-DDThh:ii:ss format
    #[serde(rename = "DateAdded", skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    /// Origin file extension
    #[serde(rename = "Source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl VerificationFileResult {
    /// Simple verification file result info
    pub fn new() -> VerificationFileResult {
        VerificationFileResult {
            verification_id: None,
            filename: None,
            verification_status: None,
            file_upload_result: None,
            date_added: None,
            source: None,
        }
    }
}

