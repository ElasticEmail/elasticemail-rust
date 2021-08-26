# \StatisticsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**statistics_campaigns_by_name_get**](StatisticsApi.md#statistics_campaigns_by_name_get) | **GET** /statistics/campaigns/{name} | Load Campaign Stats
[**statistics_campaigns_get**](StatisticsApi.md#statistics_campaigns_get) | **GET** /statistics/campaigns | Load Campaigns Stats
[**statistics_channels_by_name_get**](StatisticsApi.md#statistics_channels_by_name_get) | **GET** /statistics/channels/{name} | Load Channel Stats
[**statistics_channels_get**](StatisticsApi.md#statistics_channels_get) | **GET** /statistics/channels | Load Channels Stats
[**statistics_get**](StatisticsApi.md#statistics_get) | **GET** /statistics | Load Statistics



## statistics_campaigns_by_name_get

> crate::models::ChannelLogStatusSummary statistics_campaigns_by_name_get(name)
Load Campaign Stats

Retrieve stats of an existing campaign. Required Access Level: ViewChannels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the campaign to get. | [required] |

### Return type

[**crate::models::ChannelLogStatusSummary**](ChannelLogStatusSummary.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_campaigns_get

> Vec<crate::models::ChannelLogStatusSummary> statistics_campaigns_get(limit, offset)
Load Campaigns Stats

Returns a list of your Campaigns' stats. Required Access Level: ViewChannels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<crate::models::ChannelLogStatusSummary>**](ChannelLogStatusSummary.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_channels_by_name_get

> crate::models::ChannelLogStatusSummary statistics_channels_by_name_get(name)
Load Channel Stats

Retrieve an existing channel stats. Required Access Level: ViewChannels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | The name of the channel to get. | [required] |

### Return type

[**crate::models::ChannelLogStatusSummary**](ChannelLogStatusSummary.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_channels_get

> Vec<crate::models::ChannelLogStatusSummary> statistics_channels_get(limit, offset)
Load Channels Stats

Returns a list of your Channels' stats. Required Access Level: ViewChannels

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<crate::models::ChannelLogStatusSummary>**](ChannelLogStatusSummary.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## statistics_get

> crate::models::LogStatusSummary statistics_get(from, to)
Load Statistics

Returns basic statistics. Required Access Level: ViewReports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from** | **String** | Starting date for search in YYYY-MM-DDThh:mm:ss format. | [required] |
**to** | Option<**String**> | Ending date for search in YYYY-MM-DDThh:mm:ss format. |  |

### Return type

[**crate::models::LogStatusSummary**](LogStatusSummary.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

