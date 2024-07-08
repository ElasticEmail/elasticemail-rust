# \SubAccountsApi

All URIs are relative to *https://api.elasticemail.com/v4*

Method | HTTP request | Description
------------- | ------------- | -------------
[**subaccounts_by_email_credits_patch**](SubAccountsApi.md#subaccounts_by_email_credits_patch) | **PATCH** /subaccounts/{email}/credits | Add, Subtract Email Credits
[**subaccounts_by_email_delete**](SubAccountsApi.md#subaccounts_by_email_delete) | **DELETE** /subaccounts/{email} | Delete SubAccount
[**subaccounts_by_email_get**](SubAccountsApi.md#subaccounts_by_email_get) | **GET** /subaccounts/{email} | Load SubAccount
[**subaccounts_by_email_settings_email_put**](SubAccountsApi.md#subaccounts_by_email_settings_email_put) | **PUT** /subaccounts/{email}/settings/email | Update SubAccount Email Settings
[**subaccounts_get**](SubAccountsApi.md#subaccounts_get) | **GET** /subaccounts | Load SubAccounts
[**subaccounts_post**](SubAccountsApi.md#subaccounts_post) | **POST** /subaccounts | Add SubAccount



## subaccounts_by_email_credits_patch

> subaccounts_by_email_credits_patch(email, subaccount_email_credits_payload)
Add, Subtract Email Credits

Update email credits of a subaccount by the given amount. Required Access Level: ModifySubAccounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address of Sub-Account | [required] |
**subaccount_email_credits_payload** | [**SubaccountEmailCreditsPayload**](SubaccountEmailCreditsPayload.md) | Amount of email credits to add or subtract from the current SubAccount email credits pool (positive or negative value) | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subaccounts_by_email_delete

> subaccounts_by_email_delete(email)
Delete SubAccount

Deletes specified SubAccount. An email will be sent to confirm this change. Required Access Level: ModifySubAccounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address of Sub-Account | [required] |

### Return type

 (empty response body)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subaccounts_by_email_get

> models::SubAccountInfo subaccounts_by_email_get(email)
Load SubAccount

Returns details for the specified SubAccount. Required Access Level: ViewSubAccounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address of Sub-Account | [required] |

### Return type

[**models::SubAccountInfo**](SubAccountInfo.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subaccounts_by_email_settings_email_put

> models::SubaccountEmailSettings subaccounts_by_email_settings_email_put(email, subaccount_email_settings)
Update SubAccount Email Settings

Update SubAccount email settings. Required Access Level: ModifySubAccounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** |  | [required] |
**subaccount_email_settings** | [**SubaccountEmailSettings**](SubaccountEmailSettings.md) | Updated Email Settings | [required] |

### Return type

[**models::SubaccountEmailSettings**](SubaccountEmailSettings.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subaccounts_get

> Vec<models::SubAccountInfo> subaccounts_get(limit, offset)
Load SubAccounts

Returns a list of all your SubAccounts. Required Access Level: ViewSubAccounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Maximum number of returned items. |  |
**offset** | Option<**i32**> | How many items should be returned ahead. |  |

### Return type

[**Vec<models::SubAccountInfo>**](SubAccountInfo.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## subaccounts_post

> models::SubAccountInfo subaccounts_post(subaccount_payload)
Add SubAccount

Add a new SubAccount to your Account. To receive an access token for this SubAccount, make a POST security/apikeys request using the 'subaccount' parameter. Required Access Level: ModifySubAccounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**subaccount_payload** | [**SubaccountPayload**](SubaccountPayload.md) |  | [required] |

### Return type

[**models::SubAccountInfo**](SubAccountInfo.md)

### Authorization

[apikey](../README.md#apikey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

