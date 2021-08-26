# \FilesApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**files_by_name_delete**](FilesApi.md#files_by_name_delete) | **DELETE** /files/{name} | Delete File
[**files_by_name_get**](FilesApi.md#files_by_name_get) | **GET** /files/{name} | Download File
[**files_by_name_info_get**](FilesApi.md#files_by_name_info_get) | **GET** /files/{name}/info | Load File Details
[**files_get**](FilesApi.md#files_get) | **GET** /files | List Files
[**files_post**](FilesApi.md#files_post) | **POST** /files | Upload File



## files_by_name_delete

> files_by_name_delete(name)
Delete File

Permanently deletes the file from your Account. Required Access Level: ModifyFiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of your file including extension. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_by_name_get

> std::path::PathBuf files_by_name_get(name)
Download File

Gets content of the specified File. Required Access Level: ViewFiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of your file including extension. | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/_*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_by_name_info_get

> crate::models::FileInfo files_by_name_info_get(name)
Load File Details

Returns the specified File's details. Required Access Level: ViewFiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of your file including extension. | [required] |

### Return type

[**crate::models::FileInfo**](FileInfo.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_get

> Vec<crate::models::FileInfo> files_get(limit, offset)
List Files

Returns a list of all your available files. Required Access Level: ViewFiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<crate::models::FileInfo>**](FileInfo.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## files_post

> crate::models::FileInfo files_post(file_payload, expires_after_days)
Upload File

Uploads selected file to the server. Required Access Level: ModifyFiles

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_payload** | [**FilePayload**](FilePayload.md) |  | [required] |
**expires_after_days** | Option<**i32**> | After how many days should the file be deleted. |  |

### Return type

[**crate::models::FileInfo**](FileInfo.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

