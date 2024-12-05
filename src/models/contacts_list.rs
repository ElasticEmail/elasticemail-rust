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

/// ContactsList : List of Lists, with detailed data about its contents.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContactsList {
    /// Name of your list.
    #[serde(rename = "ListName", skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    /// ID code of list. Please note that this is different from the listid field.
    #[serde(rename = "PublicListID", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub public_list_id: Option<Option<String>>,
    /// Date of creation in YYYY-MM-DDThh:ii:ss format
    #[serde(rename = "DateAdded", skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    /// True: Allow unsubscribing from this list. Otherwise, false
    #[serde(rename = "AllowUnsubscribe", skip_serializing_if = "Option::is_none")]
    pub allow_unsubscribe: Option<bool>,
}

impl ContactsList {
    /// List of Lists, with detailed data about its contents.
    pub fn new() -> ContactsList {
        ContactsList {
            list_name: None,
            public_list_id: None,
            date_added: None,
            allow_unsubscribe: None,
        }
    }
}

