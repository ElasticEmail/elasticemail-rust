# \WebhookApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**webhook_by_publicid_delete**](WebhookApi.md#webhook_by_publicid_delete) | **DELETE** /webhook/{publicid} | Delete Webhook
[**webhook_by_publicid_get**](WebhookApi.md#webhook_by_publicid_get) | **GET** /webhook/{publicid} | Load Webhook
[**webhook_by_publicid_put**](WebhookApi.md#webhook_by_publicid_put) | **PUT** /webhook/{publicid} | Update Webhook
[**webhook_get**](WebhookApi.md#webhook_get) | **GET** /webhook | Load Webhooks
[**webhook_post**](WebhookApi.md#webhook_post) | **POST** /webhook | Add Webhook



## webhook_by_publicid_delete

> webhook_by_publicid_delete(publicid)
Delete Webhook

Delete the specified notifications webhook. Required Access Level: ModifyWebNotifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**publicid** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_by_publicid_get

> models::Webhook webhook_by_publicid_get(publicid)
Load Webhook

Load notifications webhook details. Required Access Level: ViewWebNotifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**publicid** | **String** |  | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_by_publicid_put

> models::Webhook webhook_by_publicid_put(publicid, webhook_update_payload)
Update Webhook

Update notification webhook. Required Access Level: ModifyWebNotifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**publicid** | **String** |  | [required] |
**webhook_update_payload** | [**WebhookUpdatePayload**](WebhookUpdatePayload.md) |  | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_get

> Vec<models::Webhook> webhook_get(limit, offset)
Load Webhooks

Returns a list of notification webhooks. Required Access Level: ViewWebNotifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<models::Webhook>**](Webhook.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## webhook_post

> models::Webhook webhook_post(webhook_create_payload)
Add Webhook

Add a notification webhook. Required Access Level: ModifyWebNotifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**webhook_create_payload** | [**WebhookCreatePayload**](WebhookCreatePayload.md) |  | [required] |

### Return type

[**models::Webhook**](Webhook.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

