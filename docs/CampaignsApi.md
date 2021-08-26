# \CampaignsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**campaigns_by_name_delete**](CampaignsApi.md#campaigns_by_name_delete) | **DELETE** /campaigns/{name} | Delete Campaign
[**campaigns_by_name_get**](CampaignsApi.md#campaigns_by_name_get) | **GET** /campaigns/{name} | Load Campaign
[**campaigns_by_name_put**](CampaignsApi.md#campaigns_by_name_put) | **PUT** /campaigns/{name} | Update Campaign
[**campaigns_get**](CampaignsApi.md#campaigns_get) | **GET** /campaigns | Load Campaigns
[**campaigns_post**](CampaignsApi.md#campaigns_post) | **POST** /campaigns | Add Campaign



## campaigns_by_name_delete

> campaigns_by_name_delete(name)
Delete Campaign

Delete the specific campaign.  This does not cancel in progress email, see Cancel In Progress. Required Access Level: ModifyCampaigns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of Campaign to delete | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## campaigns_by_name_get

> crate::models::Campaign campaigns_by_name_get(name)
Load Campaign

Returns the specified campaign details. Required Access Level: ViewCampaigns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of Campaign to get | [required] |

### Return type

[**crate::models::Campaign**](Campaign.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## campaigns_by_name_put

> crate::models::Campaign campaigns_by_name_put(name, campaign)
Update Campaign

Updates a previously added campaign.  Only Active and Paused campaigns can be updated. Required Access Level: ModifyCampaigns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of Campaign to update | [required] |
**campaign** | [**Campaign**](Campaign.md) | JSON representation of a campaign | [required] |

### Return type

[**crate::models::Campaign**](Campaign.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## campaigns_get

> Vec<crate::models::Campaign> campaigns_get(search, offset, limit)
Load Campaigns

Returns a list all of your campaigns. Limited to 1000 results. Required Access Level: ViewCampaigns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Text fragment used for searching in Campaign name (using the 'contains' rule) |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |
**limit** | Option<**i32**> | Maximum number of returned items. |  |

### Return type

[**Vec<crate::models::Campaign>**](Campaign.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## campaigns_post

> crate::models::Campaign campaigns_post(campaign)
Add Campaign

Add a campaign for processing. Required Access Level: ModifyCampaigns

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**campaign** | [**Campaign**](Campaign.md) | JSON representation of a campaign | [required] |

### Return type

[**crate::models::Campaign**](Campaign.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

