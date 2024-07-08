# \SegmentsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**segments_by_name_delete**](SegmentsApi.md#segments_by_name_delete) | **DELETE** /segments/{name} | Delete Segment
[**segments_by_name_get**](SegmentsApi.md#segments_by_name_get) | **GET** /segments/{name} | Load Segment
[**segments_by_name_put**](SegmentsApi.md#segments_by_name_put) | **PUT** /segments/{name} | Update Segment
[**segments_get**](SegmentsApi.md#segments_get) | **GET** /segments | Load Segments
[**segments_post**](SegmentsApi.md#segments_post) | **POST** /segments | Add Segment



## segments_by_name_delete

> segments_by_name_delete(name)
Delete Segment

Delete an existing segment. Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of your segment. | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## segments_by_name_get

> models::Segment segments_by_name_get(name)
Load Segment

Returns details for the specified segment. Required Access Level: ViewContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of the segment you want to load. Will load all contacts if the 'All Contacts' name has been provided | [required] |

### Return type

[**models::Segment**](Segment.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## segments_by_name_put

> models::Segment segments_by_name_put(name, segment_payload)
Update Segment

Rename or change RULE for your segment. Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name of your segment. | [required] |
**segment_payload** | [**SegmentPayload**](SegmentPayload.md) |  | [required] |

### Return type

[**models::Segment**](Segment.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## segments_get

> Vec<models::Segment> segments_get(limit, offset)
Load Segments

Returns a list of all your available Segments. Required Access Level: ViewContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<models::Segment>**](Segment.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## segments_post

> models::Segment segments_post(segment_payload)
Add Segment

Add a new segment, based on specified RULE. Required Access Level: ModifyContacts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**segment_payload** | [**SegmentPayload**](SegmentPayload.md) |  | [required] |

### Return type

[**models::Segment**](Segment.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

