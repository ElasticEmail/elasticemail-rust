# \EmailsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**emails_by_msgid_view_get**](EmailsApi.md#emails_by_msgid_view_get) | **GET** /emails/{msgid}/view | View Email
[**emails_by_transactionid_status_get**](EmailsApi.md#emails_by_transactionid_status_get) | **GET** /emails/{transactionid}/status | Get Status
[**emails_mergefile_post**](EmailsApi.md#emails_mergefile_post) | **POST** /emails/mergefile | Send Bulk Emails CSV
[**emails_post**](EmailsApi.md#emails_post) | **POST** /emails | Send Bulk Emails
[**emails_transactional_post**](EmailsApi.md#emails_transactional_post) | **POST** /emails/transactional | Send Transactional Email



## emails_by_msgid_view_get

> models::EmailData emails_by_msgid_view_get(msgid)
View Email

Returns email details for viewing or rendering. Required Access Level: None

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**msgid** | **String** | Message identifier | [required] |

### Return type

[**models::EmailData**](EmailData.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emails_by_transactionid_status_get

> models::EmailJobStatus emails_by_transactionid_status_get(transactionid, show_failed, show_sent, show_delivered, show_pending, show_opened, show_clicked, show_abuse, show_unsubscribed, show_errors, show_message_ids)
Get Status

Get status details of an email transaction. Required Access Level: ViewReports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transactionid** | **String** | Transaction identifier | [required] |
**show_failed** | Option<**bool**> | Include Bounced email addresses. |  |[default to false]
**show_sent** | Option<**bool**> | Include Sent email addresses. |  |[default to false]
**show_delivered** | Option<**bool**> | Include all delivered email addresses. |  |[default to false]
**show_pending** | Option<**bool**> | Include Ready to send email addresses. |  |[default to false]
**show_opened** | Option<**bool**> | Include Opened email addresses. |  |[default to false]
**show_clicked** | Option<**bool**> | Include Clicked email addresses. |  |[default to false]
**show_abuse** | Option<**bool**> | Include Reported as abuse email addresses. |  |[default to false]
**show_unsubscribed** | Option<**bool**> | Include Unsubscribed email addresses. |  |[default to false]
**show_errors** | Option<**bool**> | Include error messages for bounced emails. |  |[default to false]
**show_message_ids** | Option<**bool**> | Include all MessageIDs for this transaction |  |[default to false]

### Return type

[**models::EmailJobStatus**](EmailJobStatus.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emails_mergefile_post

> models::EmailSend emails_mergefile_post(merge_email_payload)
Send Bulk Emails CSV

Send to a list of contacts submitted in a CSV data file. The first column in the CSV must be the email address and the CSV must contain a header row. Additional fields can be included with a named header row and can be merged with the template using {merge} tags in the content.                           Example CSV:                           email, firstname, lastname              test1@gmail.com, michael, smith              test2@gmail.com, janet, smith                           Merge file must not be empty. Required Access Level: SendHttp

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**merge_email_payload** | [**MergeEmailPayload**](MergeEmailPayload.md) | Email data | [required] |

### Return type

[**models::EmailSend**](EmailSend.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emails_post

> models::EmailSend emails_post(email_message_data)
Send Bulk Emails

Send bulk merge email. Required Access Level: SendHttp

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_message_data** | [**EmailMessageData**](EmailMessageData.md) | Email data | [required] |

### Return type

[**models::EmailSend**](EmailSend.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## emails_transactional_post

> models::EmailSend emails_transactional_post(email_transactional_message_data)
Send Transactional Email

Send transactional emails (recipients will be known to each other). Required Access Level: SendHttp

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email_transactional_message_data** | [**EmailTransactionalMessageData**](EmailTransactionalMessageData.md) | Email data | [required] |

### Return type

[**models::EmailSend**](EmailSend.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

