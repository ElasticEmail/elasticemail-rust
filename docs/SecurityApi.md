# \SecurityApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**security_apikeys_by_name_delete**](SecurityApi.md#security_apikeys_by_name_delete) | **DELETE** /security/apikeys/{name} | Delete ApiKey
[**security_apikeys_by_name_get**](SecurityApi.md#security_apikeys_by_name_get) | **GET** /security/apikeys/{name} | Load ApiKey
[**security_apikeys_by_name_put**](SecurityApi.md#security_apikeys_by_name_put) | **PUT** /security/apikeys/{name} | Update ApiKey
[**security_apikeys_get**](SecurityApi.md#security_apikeys_get) | **GET** /security/apikeys | List ApiKeys
[**security_apikeys_post**](SecurityApi.md#security_apikeys_post) | **POST** /security/apikeys | Add ApiKey
[**security_smtp_by_name_delete**](SecurityApi.md#security_smtp_by_name_delete) | **DELETE** /security/smtp/{name} | Delete SMTP Credential
[**security_smtp_by_name_get**](SecurityApi.md#security_smtp_by_name_get) | **GET** /security/smtp/{name} | Load SMTP Credential
[**security_smtp_by_name_put**](SecurityApi.md#security_smtp_by_name_put) | **PUT** /security/smtp/{name} | Update SMTP Credential
[**security_smtp_get**](SecurityApi.md#security_smtp_get) | **GET** /security/smtp | List SMTP Credentials
[**security_smtp_post**](SecurityApi.md#security_smtp_post) | **POST** /security/smtp | Add SMTP Credential



## security_apikeys_by_name_delete

> security_apikeys_by_name_delete(name, subaccount)
Delete ApiKey

Delete your existing ApiKey. Required Access Level: Security

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the ApiKey | [required] |
**subaccount** | Option<**String**> | Email of the subaccount of which ApiKey should be deleted |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_apikeys_by_name_get

> models::ApiKey security_apikeys_by_name_get(name, subaccount)
Load ApiKey

Load your existing ApiKey info. Required Access Level: Security

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the ApiKey | [required] |
**subaccount** | Option<**String**> | Email of the subaccount of which ApiKey should be loaded |  |

### Return type

[**models::ApiKey**](ApiKey.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_apikeys_by_name_put

> models::ApiKey security_apikeys_by_name_put(name, api_key_payload)
Update ApiKey

Update your existing ApiKey. Required Access Level: Security

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the ApiKey | [required] |
**api_key_payload** | [**ApiKeyPayload**](ApiKeyPayload.md) |  | [required] |

### Return type

[**models::ApiKey**](ApiKey.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_apikeys_get

> Vec<models::ApiKey> security_apikeys_get(subaccount)
List ApiKeys

List all your existing ApiKeys. Required Access Level: Security

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount** | Option<**String**> | Email of the subaccount of which ApiKeys should be loaded |  |

### Return type

[**Vec<models::ApiKey>**](ApiKey.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_apikeys_post

> models::NewApiKey security_apikeys_post(api_key_payload)
Add ApiKey

Add a new ApiKey. Required Access Level: Security

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_payload** | [**ApiKeyPayload**](ApiKeyPayload.md) |  | [required] |

### Return type

[**models::NewApiKey**](NewApiKey.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_smtp_by_name_delete

> security_smtp_by_name_delete(name, subaccount)
Delete SMTP Credential

Delete your existing SMTP Credentials. Required Access Level: Security

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the SMTP Credential | [required] |
**subaccount** | Option<**String**> | Email of the subaccount of which credential should be deleted |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_smtp_by_name_get

> models::SmtpCredentials security_smtp_by_name_get(name, subaccount)
Load SMTP Credential

Load your existing SMTP Credential info. Required Access Level: Security

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the SMTP Credential | [required] |
**subaccount** | Option<**String**> | Email of the subaccount of which credential should be loaded |  |

### Return type

[**models::SmtpCredentials**](SmtpCredentials.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_smtp_by_name_put

> models::SmtpCredentials security_smtp_by_name_put(name, smtp_credentials_payload)
Update SMTP Credential

Update your existing SMTP Credentials. Required Access Level: Security

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the SMTP Credential | [required] |
**smtp_credentials_payload** | [**SmtpCredentialsPayload**](SmtpCredentialsPayload.md) |  | [required] |

### Return type

[**models::SmtpCredentials**](SmtpCredentials.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_smtp_get

> Vec<models::SmtpCredentials> security_smtp_get(subaccount)
List SMTP Credentials

List all your existing SMTP Credentials. Required Access Level: Security

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount** | Option<**String**> | Email of the subaccount of which credentials should be listed |  |

### Return type

[**Vec<models::SmtpCredentials>**](SmtpCredentials.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## security_smtp_post

> models::NewSmtpCredentials security_smtp_post(smtp_credentials_payload)
Add SMTP Credential

Add new SMTP Credential. Required Access Level: Security

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**smtp_credentials_payload** | [**SmtpCredentialsPayload**](SmtpCredentialsPayload.md) |  | [required] |

### Return type

[**models::NewSmtpCredentials**](NewSmtpCredentials.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

