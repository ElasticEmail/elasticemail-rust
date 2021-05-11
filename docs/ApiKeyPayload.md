# ApiKeyPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the ApiKey for ease of reference. | [optional]
**access_level** | Option<[**Vec<crate::models::AccessLevel>**](AccessLevel.md)> | Access level or permission to be assigned to this ApiKey. | [optional]
**expires** | Option<**String**> | Date this ApiKey expires. | [optional]
**restrict_access_to_ip_range** | Option<**Vec<String>**> | Which IPs can use this ApiKey | [optional]
**subaccount** | Option<**String**> | Email of the subaccount for which this ApiKey should be created | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


