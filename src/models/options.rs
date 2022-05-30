/*
 * Elastic Email REST API
 *
 * This API is based on the REST API architecture, allowing the user to easily manage their data with this resource-based approach.    Every API call is established on which specific request type (GET, POST, PUT, DELETE) will be used.    The API has a limit of 20 concurrent connections and a hard timeout of 600 seconds per request.    To start using this API, you will need your Access Token (available <a target=\"_blank\" href=\"https://elasticemail.com/account#/settings/new/manage-api\">here</a>). Remember to keep it safe. Required access levels are listed in the given request’s description.    Downloadable library clients can be found in our Github repository <a target=\"_blank\" href=\"https://github.com/ElasticEmail?tab=repositories&q=%22rest+api%22+in%3Areadme\">here</a>
 *
 * The version of the OpenAPI document: 4.0.0
 * Contact: support@elasticemail.com
 * Generated by: https://openapi-generator.tech
 */

/// Options : E-mail configuration



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Options {
    /// By how long should an e-mail be delayed (in minutes). Maximum is 35 days.
    #[serde(rename = "TimeOffset", skip_serializing_if = "Option::is_none")]
    pub time_offset: Option<i32>,
    /// Name of your custom IP Pool to be used in the sending process
    #[serde(rename = "PoolName", skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
    /// Name of selected channel.
    #[serde(rename = "ChannelName", skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    #[serde(rename = "Encoding", skip_serializing_if = "Option::is_none")]
    pub encoding: Option<crate::models::EncodingType>,
    /// Should the opens be tracked? If no value has been provided, Account's default setting will be used.
    #[serde(rename = "TrackOpens", skip_serializing_if = "Option::is_none")]
    pub track_opens: Option<bool>,
    /// Should the clicks be tracked? If no value has been provided, Account's default setting will be used.
    #[serde(rename = "TrackClicks", skip_serializing_if = "Option::is_none")]
    pub track_clicks: Option<bool>,
}

impl Options {
    /// E-mail configuration
    pub fn new() -> Options {
        Options {
            time_offset: None,
            pool_name: None,
            channel_name: None,
            encoding: None,
            track_opens: None,
            track_clicks: None,
        }
    }
}


