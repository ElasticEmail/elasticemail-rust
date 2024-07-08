# \ListsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lists_by_listname_contacts_get**](ListsApi.md#lists_by_listname_contacts_get) | **GET** /lists/{listname}/contacts | Load Contacts in List
[**lists_by_name_contacts_post**](ListsApi.md#lists_by_name_contacts_post) | **POST** /lists/{name}/contacts | Add Contacts to List
[**lists_by_name_contacts_remove_post**](ListsApi.md#lists_by_name_contacts_remove_post) | **POST** /lists/{name}/contacts/remove | Remove Contacts from List
[**lists_by_name_delete**](ListsApi.md#lists_by_name_delete) | **DELETE** /lists/{name} | Delete List
[**lists_by_name_get**](ListsApi.md#lists_by_name_get) | **GET** /lists/{name} | Load List
[**lists_by_name_put**](ListsApi.md#lists_by_name_put) | **PUT** /lists/{name} | Update List
[**lists_get**](ListsApi.md#lists_get) | **GET** /lists | Load Lists
[**lists_post**](ListsApi.md#lists_post) | **POST** /lists | Add List



## lists_by_listname_contacts_get

> Vec<models::Contact> lists_by_listname_contacts_get(listname, limit, offset)
Load Contacts in List

Returns a list of contacts. Required Access Level: ViewContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**listname** | **String** | Name of your list. | [required] |
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<models::Contact>**](Contact.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lists_by_name_contacts_post

> models::ContactsList lists_by_name_contacts_post(name, emails_payload)
Add Contacts to List

Add existing Contacts to specified list. Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of your list. | [required] |
**emails_payload** | [**EmailsPayload**](EmailsPayload.md) | Provide either rule or a list of emails, not both. | [required] |

### Return type

[**models::ContactsList**](ContactsList.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lists_by_name_contacts_remove_post

> lists_by_name_contacts_remove_post(name, emails_payload)
Remove Contacts from List

Remove specified Contacts from your list. Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of your list. | [required] |
**emails_payload** | [**EmailsPayload**](EmailsPayload.md) | Provide either rule or a list of emails, not both. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lists_by_name_delete

> lists_by_name_delete(name)
Delete List

Deletes List and removes all the Contacts from it (does not delete Contacts). Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of your list. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lists_by_name_get

> models::ContactsList lists_by_name_get(name)
Load List

Returns detailed information about specified list. Required Access Level: ViewContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of your list. | [required] |

### Return type

[**models::ContactsList**](ContactsList.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lists_by_name_put

> models::ContactsList lists_by_name_put(name, list_update_payload)
Update List

Update existing list. Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of your list. | [required] |
**list_update_payload** | [**ListUpdatePayload**](ListUpdatePayload.md) |  | [required] |

### Return type

[**models::ContactsList**](ContactsList.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lists_get

> Vec<models::ContactsList> lists_get(limit, offset)
Load Lists

Returns all your existing lists. Required Access Level: ViewContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<models::ContactsList>**](ContactsList.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## lists_post

> models::ContactsList lists_post(list_payload)
Add List

Add a new list. Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_payload** | [**ListPayload**](ListPayload.md) |  | [required] |

### Return type

[**models::ContactsList**](ContactsList.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

