# \InboundRouteApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**inboundroute_by_id_delete**](InboundRouteApi.md#inboundroute_by_id_delete) | **delete** /inboundroute/{id} | Delete Route
[**inboundroute_by_id_get**](InboundRouteApi.md#inboundroute_by_id_get) | **get** /inboundroute/{id} | Get Route
[**inboundroute_by_id_put**](InboundRouteApi.md#inboundroute_by_id_put) | **put** /inboundroute/{id} | Update Route
[**inboundroute_get**](InboundRouteApi.md#inboundroute_get) | **get** /inboundroute | Get Routes
[**inboundroute_order_put**](InboundRouteApi.md#inboundroute_order_put) | **put** /inboundroute/order | Update Sorting
[**inboundroute_post**](InboundRouteApi.md#inboundroute_post) | **post** /inboundroute | Create Route



## inboundroute_by_id_delete

> inboundroute_by_id_delete(id)
Delete Route

Deletes the Inbound Route. Required Access Level: ModifySettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inboundroute_by_id_get

> crate::models::InboundRoute inboundroute_by_id_get(id)
Get Route

Load an Inbound Route. Required Access Level: ViewSettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID number of your attachment | [required] |

### Return type

[**crate::models::InboundRoute**](InboundRoute.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inboundroute_by_id_put

> crate::models::InboundRoute inboundroute_by_id_put(id, inbound_payload)
Update Route

Update the Inbound Route. Required Access Level: ModifySettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**inbound_payload** | [**InboundPayload**](InboundPayload.md) |  | [required] |

### Return type

[**crate::models::InboundRoute**](InboundRoute.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inboundroute_get

> Vec<crate::models::InboundRoute> inboundroute_get()
Get Routes

Get all your Inbound Routes. Required Access Level: ViewSettings

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InboundRoute>**](InboundRoute.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inboundroute_order_put

> Vec<crate::models::InboundRoute> inboundroute_order_put(sort_order_item)
Update Sorting

Required Access Level: ViewSettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**sort_order_item** | [**Vec<crate::models::SortOrderItem>**](SortOrderItem.md) | Change the ordering of inbound routes for when matching the inbound | [required] |

### Return type

[**Vec<crate::models::InboundRoute>**](InboundRoute.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## inboundroute_post

> crate::models::InboundRoute inboundroute_post(inbound_payload)
Create Route

Create new Inbound Route. Required Access Level: ModifySettings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inbound_payload** | [**InboundPayload**](InboundPayload.md) |  | [required] |

### Return type

[**crate::models::InboundRoute**](InboundRoute.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

