# \ContactsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contacts_by_email_delete**](ContactsApi.md#contacts_by_email_delete) | **DELETE** /contacts/{email} | Delete Contact
[**contacts_by_email_get**](ContactsApi.md#contacts_by_email_get) | **GET** /contacts/{email} | Load Contact
[**contacts_by_email_put**](ContactsApi.md#contacts_by_email_put) | **PUT** /contacts/{email} | Update Contact
[**contacts_delete_post**](ContactsApi.md#contacts_delete_post) | **POST** /contacts/delete | Delete Contacts Bulk
[**contacts_export_by_id_status_get**](ContactsApi.md#contacts_export_by_id_status_get) | **GET** /contacts/export/{id}/status | Check Export Status
[**contacts_export_post**](ContactsApi.md#contacts_export_post) | **POST** /contacts/export | Export Contacts
[**contacts_get**](ContactsApi.md#contacts_get) | **GET** /contacts | Load Contacts
[**contacts_import_post**](ContactsApi.md#contacts_import_post) | **POST** /contacts/import | Upload Contacts
[**contacts_post**](ContactsApi.md#contacts_post) | **POST** /contacts | Add Contact



## contacts_by_email_delete

> contacts_by_email_delete(email)
Delete Contact

Deletes the provided contact. Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Proper email address. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contacts_by_email_get

> models::Contact contacts_by_email_get(email)
Load Contact

Load detailed contact information for specified email. Required Access Level: ViewContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Proper email address. | [required] |

### Return type

[**models::Contact**](Contact.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contacts_by_email_put

> models::Contact contacts_by_email_put(email, contact_update_payload)
Update Contact

Update selected contact. Omitted contact's fields will not be changed. Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Proper email address. | [required] |
**contact_update_payload** | [**ContactUpdatePayload**](ContactUpdatePayload.md) |  | [required] |

### Return type

[**models::Contact**](Contact.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contacts_delete_post

> contacts_delete_post(emails_payload)
Delete Contacts Bulk

Deletes provided contacts in bulk. Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**emails_payload** | [**EmailsPayload**](EmailsPayload.md) | Provide either rule or a list of emails, not both. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contacts_export_by_id_status_get

> models::ExportStatus contacts_export_by_id_status_get(id)
Check Export Status

Check the current status of the export. Required Access Level: Export

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the exported file | [required] |

### Return type

[**models::ExportStatus**](ExportStatus.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contacts_export_post

> models::ExportLink contacts_export_post(file_format, rule, emails, compression_format, file_name)
Export Contacts

Request an Export of specified Contacts. Required Access Level: Export

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_format** | Option<[**ExportFileFormats**](.md)> | Format of the exported file |  |
**rule** | Option<**String**> | Query used for filtering. |  |
**emails** | Option<[**Vec<String>**](String.md)> | Comma delimited list of contact emails |  |
**compression_format** | Option<[**CompressionFormat**](.md)> | FileResponse compression format. None or Zip. |  |
**file_name** | Option<**String**> | Name of your file including extension. |  |

### Return type

[**models::ExportLink**](ExportLink.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contacts_get

> Vec<models::Contact> contacts_get(limit, offset)
Load Contacts

Returns a list of contacts. Required Access Level: ViewContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## contacts_import_post

> contacts_import_post(list_name, encoding_name, file_url, file)
Upload Contacts

Upload contacts from a file. Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_name** | Option<**String**> | Name of an existing list to add these contacts to |  |
**encoding_name** | Option<**String**> | In what encoding the file is uploaded |  |
**file_url** | Option<**String**> | Optional url of csv to import |  |
**file** | Option<**std::path::PathBuf**> |  |  |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contacts_post

> Vec<models::Contact> contacts_post(contact_payload, listnames)
Add Contact

Add new Contacts to your Lists. Up to 1000 can be added (for more please refer to the import request). Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_payload** | [**Vec<models::ContactPayload>**](ContactPayload.md) |  | [required] |
**listnames** | Option<[**Vec<String>**](String.md)> | Names of lists to which the uploaded contacts should be added to |  |

### Return type

[**Vec<models::Contact>**](Contact.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

