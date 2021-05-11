# \TemplatesApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**templates_by_name_delete**](TemplatesApi.md#templates_by_name_delete) | **delete** /templates/{name} | Delete Template
[**templates_by_name_get**](TemplatesApi.md#templates_by_name_get) | **get** /templates/{name} | Load Template
[**templates_by_name_put**](TemplatesApi.md#templates_by_name_put) | **put** /templates/{name} | Update Template
[**templates_get**](TemplatesApi.md#templates_get) | **get** /templates | Load Templates
[**templates_post**](TemplatesApi.md#templates_post) | **post** /templates | Add Template



## templates_by_name_delete

> templates_by_name_delete(name)
Delete Template

Delete template with the specified name. Required Access Level: ModifyTemplates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of template. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## templates_by_name_get

> crate::models::Template templates_by_name_get(name)
Load Template

Load detailed information of the specified template. Required Access Level: ViewTemplates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of template. | [required] |

### Return type

[**crate::models::Template**](Template.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## templates_by_name_put

> crate::models::Template templates_by_name_put(name, template_payload)
Update Template

Update existing template, overwriting existing data. Required Access Level: ModifyTemplates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of template. | [required] |
**template_payload** | [**TemplatePayload**](TemplatePayload.md) |  | [required] |

### Return type

[**crate::models::Template**](Template.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## templates_get

> Vec<crate::models::Template> templates_get(scope_type, template_types, limit, offset)
Load Templates

Returns a list of templates for the specified type. Required Access Level: ViewTemplates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope_type** | [**Vec<crate::models::TemplateScopeType>**](crate::models::TemplateScopeType.md) | Return templates with specified scope only | [required] |
**template_types** | Option<[**Vec<crate::models::TemplateType>**](crate::models::TemplateType.md)> | Return templates with specified type only |  |
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<crate::models::Template>**](Template.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## templates_post

> crate::models::Template templates_post(template_payload)
Add Template

Add a new Template. Required Access Level: ModifyTemplates

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**template_payload** | [**TemplatePayload**](TemplatePayload.md) |  | [required] |

### Return type

[**crate::models::Template**](Template.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

