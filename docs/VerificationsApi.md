# \VerificationsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**verifications_by_email_delete**](VerificationsApi.md#verifications_by_email_delete) | **DELETE** /verifications/{email} | Delete Email Verification Result
[**verifications_by_email_get**](VerificationsApi.md#verifications_by_email_get) | **GET** /verifications/{email} | Get Email Verification Result
[**verifications_by_email_post**](VerificationsApi.md#verifications_by_email_post) | **POST** /verifications/{email} | Verify Email
[**verifications_files_by_id_delete**](VerificationsApi.md#verifications_files_by_id_delete) | **DELETE** /verifications/files/{id} | Delete File Verification Result
[**verifications_files_by_id_result_download_get**](VerificationsApi.md#verifications_files_by_id_result_download_get) | **GET** /verifications/files/{id}/result/download | Download File Verification Result
[**verifications_files_by_id_result_get**](VerificationsApi.md#verifications_files_by_id_result_get) | **GET** /verifications/files/{id}/result | Get Detailed File Verification Result
[**verifications_files_by_id_verification_post**](VerificationsApi.md#verifications_files_by_id_verification_post) | **POST** /verifications/files/{id}/verification | Start verification
[**verifications_files_post**](VerificationsApi.md#verifications_files_post) | **POST** /verifications/files | Upload File with Emails
[**verifications_files_result_get**](VerificationsApi.md#verifications_files_result_get) | **GET** /verifications/files/result | Get Files Verification Results
[**verifications_get**](VerificationsApi.md#verifications_get) | **GET** /verifications | Get Emails Verification Results



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

> models::EmailValidationResult verifications_by_email_get(email)
Get Email Verification Result

Returns a result of verified email. Required Access Level: VerifyEmails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address to view verification result of | [required] |

### Return type

[**models::EmailValidationResult**](EmailValidationResult.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_by_email_post

> models::EmailValidationResult verifications_by_email_post(email)
Verify Email

Verify single email address and returns result of verification. Required Access Level: VerifyEmails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address to verify | [required] |

### Return type

[**models::EmailValidationResult**](EmailValidationResult.md)

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
- **Accept**: application/*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_files_by_id_result_get

> models::VerificationFileResultDetails verifications_files_by_id_result_get(id, limit, offset)
Get Detailed File Verification Result

Returns status and results (if verified) of file with given ID. Required Access Level: VerifyEmails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the Verification to display status of | [required] |
**limit** | Option<**i32**> | Maximum number of returned email verification results |  |
**offset** | Option<**i32**> | How many result items should be returned ahead |  |

### Return type

[**models::VerificationFileResultDetails**](VerificationFileResultDetails.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_files_by_id_verification_post

> verifications_files_by_id_verification_post(id)
Start verification

Start a verification of the previously uploaded file with emails. Required Access Level: VerifyEmails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | File ID to start verification | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_files_post

> models::VerificationFileResult verifications_files_post(file)
Upload File with Emails

Uploads a CSV file with list of emails that can then be triggered for verification. An 'email' column is required. Required Access Level: VerifyEmails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file** | Option<**std::path::PathBuf**> |  |  |

### Return type

[**models::VerificationFileResult**](VerificationFileResult.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_files_result_get

> Vec<models::VerificationFileResult> verifications_files_result_get()
Get Files Verification Results

Returns a list of uploaded files, their statuses and results. Required Access Level: VerifyEmails

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::VerificationFileResult>**](VerificationFileResult.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verifications_get

> Vec<models::EmailValidationResult> verifications_get(limit, offset)
Get Emails Verification Results

Returns a results of all verified single emails. Required Access Level: VerifyEmails

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<models::EmailValidationResult>**](EmailValidationResult.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

