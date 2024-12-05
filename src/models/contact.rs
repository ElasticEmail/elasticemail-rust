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

/// Contact : Contact
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    /// Proper email address.
    #[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<models::ContactStatus>,
    /// First name.
    #[serde(rename = "FirstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(rename = "LastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// A key-value collection of custom contact fields which can be used in the system.
    #[serde(rename = "CustomFields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Consent", skip_serializing_if = "Option::is_none")]
    pub consent: Option<Box<models::ConsentData>>,
    #[serde(rename = "Source", skip_serializing_if = "Option::is_none")]
    pub source: Option<models::ContactSource>,
    /// Date of creation in YYYY-MM-DDThh:ii:ss format
    #[serde(rename = "DateAdded", skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    /// Last change date
    #[serde(rename = "DateUpdated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<Option<String>>,
    /// Date of last status change.
    #[serde(rename = "StatusChangeDate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_change_date: Option<Option<String>>,
    #[serde(rename = "Activity", skip_serializing_if = "Option::is_none")]
    pub activity: Option<Box<models::ContactActivity>>,
}

impl Contact {
    /// Contact
    pub fn new() -> Contact {
        Contact {
            email: None,
            status: None,
            first_name: None,
            last_name: None,
            custom_fields: None,
            consent: None,
            source: None,
            date_added: None,
            date_updated: None,
            status_change_date: None,
            activity: None,
        }
    }
}

