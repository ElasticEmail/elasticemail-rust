# \EventsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**events_by_transactionid_get**](EventsApi.md#events_by_transactionid_get) | **GET** /events/{transactionid} | Load Email Events
[**events_channels_by_name_export_post**](EventsApi.md#events_channels_by_name_export_post) | **POST** /events/channels/{name}/export | Export Channel Events
[**events_channels_by_name_get**](EventsApi.md#events_channels_by_name_get) | **GET** /events/channels/{name} | Load Channel Events
[**events_channels_export_by_id_status_get**](EventsApi.md#events_channels_export_by_id_status_get) | **GET** /events/channels/export/{id}/status | Check Channel Export Status
[**events_export_by_id_status_get**](EventsApi.md#events_export_by_id_status_get) | **GET** /events/export/{id}/status | Check Export Status
[**events_export_post**](EventsApi.md#events_export_post) | **POST** /events/export | Export Events
[**events_get**](EventsApi.md#events_get) | **GET** /events | Load Events



## events_by_transactionid_get

> Vec<crate::models::RecipientEvent> events_by_transactionid_get(transactionid, from, to, order_by, limit, offset)
Load Email Events

Returns a log of delivery events for the specific transaction ID. Required Access Level: ViewReports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transactionid** | **String** | ID number of transaction | [required] |
**from** | Option<**String**> | Starting date for search in YYYY-MM-DDThh:mm:ss format. |  |
**to** | Option<**String**> | Ending date for search in YYYY-MM-DDThh:mm:ss format. |  |
**order_by** | Option<[**EventsOrderBy**](.md)> |  |  |
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<crate::models::RecipientEvent>**](RecipientEvent.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_channels_by_name_export_post

> crate::models::ExportLink events_channels_by_name_export_post(name, event_types, from, to, file_format, compression_format, file_name)
Export Channel Events

Export delivery events log information to the specified file format. Required Access Level: Export

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of selected channel. | [required] |
**event_types** | Option<[**Vec<crate::models::EventType>**](crate::models::EventType.md)> | Types of Events to return |  |
**from** | Option<**String**> | Starting date for search in YYYY-MM-DDThh:mm:ss format. |  |
**to** | Option<**String**> | Ending date for search in YYYY-MM-DDThh:mm:ss format. |  |
**file_format** | Option<[**ExportFileFormats**](.md)> | Format of the exported file |  |
**compression_format** | Option<[**CompressionFormat**](.md)> | FileResponse compression format. None or Zip. |  |
**file_name** | Option<**String**> | Name of your file including extension. |  |

### Return type

[**crate::models::ExportLink**](ExportLink.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_channels_by_name_get

> Vec<crate::models::RecipientEvent> events_channels_by_name_get(name, event_types, from, to, order_by, limit, offset)
Load Channel Events

Returns a log of delivery events filtered by specified parameters. Required Access Level: ViewReports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of selected channel. | [required] |
**event_types** | Option<[**Vec<crate::models::EventType>**](crate::models::EventType.md)> | Types of Events to return |  |
**from** | Option<**String**> | Starting date for search in YYYY-MM-DDThh:mm:ss format. |  |
**to** | Option<**String**> | Ending date for search in YYYY-MM-DDThh:mm:ss format. |  |
**order_by** | Option<[**EventsOrderBy**](.md)> |  |  |
**limit** | Option<**i32**> | How many items to load. Maximum for this request is 1000 items |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<crate::models::RecipientEvent>**](RecipientEvent.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_channels_export_by_id_status_get

> crate::models::ExportStatus events_channels_export_by_id_status_get(id)
Check Channel Export Status

Check the current status of the channel export. Required Access Level: Export

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the exported file | [required] |

### Return type

[**crate::models::ExportStatus**](ExportStatus.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_export_by_id_status_get

> crate::models::ExportStatus events_export_by_id_status_get(id)
Check Export Status

Check the current status of the export. Required Access Level: Export

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the exported file | [required] |

### Return type

[**crate::models::ExportStatus**](ExportStatus.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_export_post

> crate::models::ExportLink events_export_post(event_types, from, to, file_format, compression_format, file_name)
Export Events

Export delivery events log information to the specified file format. Required Access Level: Export

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_types** | Option<[**Vec<crate::models::EventType>**](crate::models::EventType.md)> | Types of Events to return |  |
**from** | Option<**String**> | Starting date for search in YYYY-MM-DDThh:mm:ss format. |  |
**to** | Option<**String**> | Ending date for search in YYYY-MM-DDThh:mm:ss format. |  |
**file_format** | Option<[**ExportFileFormats**](.md)> | Format of the exported file |  |
**compression_format** | Option<[**CompressionFormat**](.md)> | FileResponse compression format. None or Zip. |  |
**file_name** | Option<**String**> | Name of your file including extension. |  |

### Return type

[**crate::models::ExportLink**](ExportLink.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_get

> Vec<crate::models::RecipientEvent> events_get(event_types, from, to, order_by, limit, offset)
Load Events

Returns a log of delivery events filtered by specified parameters. Required Access Level: ViewReports

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**event_types** | Option<[**Vec<crate::models::EventType>**](crate::models::EventType.md)> | Types of Events to return |  |
**from** | Option<**String**> | Starting date for search in YYYY-MM-DDThh:mm:ss format. |  |
**to** | Option<**String**> | Ending date for search in YYYY-MM-DDThh:mm:ss format. |  |
**order_by** | Option<[**EventsOrderBy**](.md)> |  |  |
**limit** | Option<**i32**> | How many items to load. Maximum for this request is 1000 items |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<crate::models::RecipientEvent>**](RecipientEvent.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

