# \DomainsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**domains_by_domain_delete**](DomainsApi.md#domains_by_domain_delete) | **DELETE** /domains/{domain} | Delete Domain
[**domains_by_domain_get**](DomainsApi.md#domains_by_domain_get) | **GET** /domains/{domain} | Load Domain
[**domains_by_domain_put**](DomainsApi.md#domains_by_domain_put) | **PUT** /domains/{domain} | Update Domain
[**domains_by_domain_restricted_get**](DomainsApi.md#domains_by_domain_restricted_get) | **GET** /domains/{domain}/restricted | Check for domain restriction
[**domains_by_domain_verification_put**](DomainsApi.md#domains_by_domain_verification_put) | **PUT** /domains/{domain}/verification | Verify Domain
[**domains_by_email_default_patch**](DomainsApi.md#domains_by_email_default_patch) | **PATCH** /domains/{email}/default | Set Default
[**domains_get**](DomainsApi.md#domains_get) | **GET** /domains | Load Domains
[**domains_post**](DomainsApi.md#domains_post) | **POST** /domains | Add Domain



## domains_by_domain_delete

> domains_by_domain_delete(domain)
Delete Domain

Deletes configured domain from Account. Required Access Level: ModifySettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | Name of the given domain | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_by_domain_get

> models::DomainData domains_by_domain_get(domain)
Load Domain

Retrieve a domain configured for this Account. Required Access Level: ViewSettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | Name of the given domain | [required] |

### Return type

[**models::DomainData**](DomainData.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_by_domain_put

> models::DomainDetail domains_by_domain_put(domain, domain_update_payload)
Update Domain

Updates the specified domain. Required Access Level: ModifySettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | Name of the given domain | [required] |
**domain_update_payload** | [**DomainUpdatePayload**](DomainUpdatePayload.md) | Updated Domain resource | [required] |

### Return type

[**models::DomainDetail**](DomainDetail.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_by_domain_restricted_get

> bool domains_by_domain_restricted_get(domain)
Check for domain restriction

Checking if domain is from free provider, or restricted. Required Access Level: ViewSettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | Name of the given domain | [required] |

### Return type

**bool**

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_by_domain_verification_put

> models::DomainData domains_by_domain_verification_put(domain, body)
Verify Domain

Verifies that required DNS records exist for specified domain. Required Access Level: ModifySettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain** | **String** | Name of the given domain | [required] |
**body** | **String** | Tracking type used in the Tracking verification | [required] |

### Return type

[**models::DomainData**](DomainData.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_by_email_default_patch

> models::DomainDetail domains_by_email_default_patch(email)
Set Default

Sets a verified email address as default sender. Required Access Level: ModifySettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Default email sender, example: mail@yourdomain.com | [required] |

### Return type

[**models::DomainDetail**](DomainDetail.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_get

> Vec<models::DomainDetail> domains_get()
Load Domains

Returns a list of all domains configured for this Account. Required Access Level: ViewSettings

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::DomainDetail>**](DomainDetail.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_post

> models::DomainDetail domains_post(domain_payload)
Add Domain

Add new domain to Account. Required Access Level: ModifySettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_payload** | [**DomainPayload**](DomainPayload.md) | Domain to add | [required] |

### Return type

[**models::DomainDetail**](DomainDetail.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

