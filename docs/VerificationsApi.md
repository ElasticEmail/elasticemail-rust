# \VerificationsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**verifications_by_email_delete**](VerificationsApi.md#verifications_by_email_delete) | **delete** /verifications/{email} | Delete Email Verification Result
[**verifications_by_email_get**](VerificationsApi.md#verifications_by_email_get) | **get** /verifications/{email} | Get Email Verification Result
[**verifications_by_email_post**](VerificationsApi.md#verifications_by_email_post) | **post** /verifications/{email} | Verify Email
[**verifications_files_by_id_delete**](VerificationsApi.md#verifications_files_by_id_delete) | **delete** /verifications/files/{id} | Delete File Verification Result
[**verifications_files_by_id_result_download_get**](VerificationsApi.md#verifications_files_by_id_result_download_get) | **get** /verifications/files/{id}/result/download | Download File Verification Result
[**verifications_files_by_id_result_get**](VerificationsApi.md#verifications_files_by_id_result_get) | **get** /verifications/files/{id}/result | Get Detailed File Verification Result
[**verifications_files_post**](VerificationsApi.md#verifications_files_post) | **post** /verifications/files | Verify From File
[**verifications_files_result_get**](VerificationsApi.md#verifications_files_result_get) | **get** /verifications/files/result | Get Simple Files Verification Results
[**verifications_get**](VerificationsApi.md#verifications_get) | **get** /verifications | Get Emails Verification Results



## verifications_by_email_delete

> verifications_by_email_delete(email)
Delete Email Verification Result

Delete a result with given email if exists. Required Access Level: VerifyEmails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address to verification | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_by_email_get

> crate::models::EmailValidationResult verifications_by_email_get(email)
Get Email Verification Result

Returns a result of verified email. Required Access Level: ViewEmailVerifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address to view verification result of | [required] |

### Return type

[**crate::models::EmailValidationResult**](EmailValidationResult.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_by_email_post

> crate::models::EmailValidationResult verifications_by_email_post(email)
Verify Email

Verify single email address and returns result of verification. Required Access Level: VerifyEmails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address to verify | [required] |

### Return type

[**crate::models::EmailValidationResult**](EmailValidationResult.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_files_by_id_delete

> verifications_files_by_id_delete(id)
Delete File Verification Result

Delete Verification Results if they exist. Required Access Level: VerifyEmails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the exported file | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_files_by_id_result_download_get

> std::path::PathBuf verifications_files_by_id_result_download_get(id)
Download File Verification Result

Download verification results as a ZIP file. Required Access Level: VerifyEmails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Verification ID to download | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/_*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_files_by_id_result_get

> crate::models::VerificationFileResultDetails verifications_files_by_id_result_get(id, limit, offset)
Get Detailed File Verification Result

Returns status and results (if verified) of file with given ID. Required Access Level: ViewEmailVerifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the Verification to display status of | [required] |
**limit** | Option<**i32**> | Maximum number of returned email verification results |  |
**offset** | Option<**i32**> | How many result items should be returned ahead |  |

### Return type

[**crate::models::VerificationFileResultDetails**](VerificationFileResultDetails.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_files_post

> crate::models::VerificationFileResult verifications_files_post(file)
Verify From File

Uploads a CSV file with list of emails to verify. An 'email' column is required. Required Access Level: VerifyEmails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**crate::models::VerificationFileResult**](VerificationFileResult.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_files_result_get

> Vec<crate::models::VerificationFileResult> verifications_files_result_get()
Get Simple Files Verification Results

Returns a list of uploaded files, their statuses and results. Required Access Level: ViewEmailVerifications

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::VerificationFileResult>**](VerificationFileResult.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_get

> Vec<crate::models::EmailValidationResult> verifications_get(limit, offset)
Get Emails Verification Results

Returns a results of all verified single emails. Required Access Level: ViewEmailVerifications

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<crate::models::EmailValidationResult>**](EmailValidationResult.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
