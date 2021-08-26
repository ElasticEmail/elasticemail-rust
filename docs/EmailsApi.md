# \EmailsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**emails_by_msgid_view_get**](EmailsApi.md#emails_by_msgid_view_get) | **GET** /emails/{msgid}/view | View Email
[**emails_mergefile_post**](EmailsApi.md#emails_mergefile_post) | **POST** /emails/mergefile | Send Bulk Emails CSV
[**emails_post**](EmailsApi.md#emails_post) | **POST** /emails | Send Bulk Emails
[**emails_transactional_post**](EmailsApi.md#emails_transactional_post) | **POST** /emails/transactional | Send Transactional Email



## emails_by_msgid_view_get

> crate::models::EmailData emails_by_msgid_view_get(msgid)
View Email

Returns email details for viewing or rendering. Required Access Level: None

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**msgid** | **String** | Message identifier | [required] |

### Return type

[**crate::models::EmailData**](EmailData.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emails_mergefile_post

> crate::models::EmailSend emails_mergefile_post(merge_email_payload)
Send Bulk Emails CSV

Send bulk merge email. Required Access Level: SendHttp

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**merge_email_payload** | [**MergeEmailPayload**](MergeEmailPayload.md) | Email data | [required] |

### Return type

[**crate::models::EmailSend**](EmailSend.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emails_post

> crate::models::EmailSend emails_post(email_message_data)
Send Bulk Emails

Send bulk merge email. Required Access Level: SendHttp

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_message_data** | [**EmailMessageData**](EmailMessageData.md) | Email data | [required] |

### Return type

[**crate::models::EmailSend**](EmailSend.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emails_transactional_post

> crate::models::EmailSend emails_transactional_post(email_transactional_message_data)
Send Transactional Email

Send transactional emails (recipients will be known to each other). Required Access Level: SendHttp

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_transactional_message_data** | [**EmailTransactionalMessageData**](EmailTransactionalMessageData.md) | Email data | [required] |

### Return type

[**crate::models::EmailSend**](EmailSend.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

