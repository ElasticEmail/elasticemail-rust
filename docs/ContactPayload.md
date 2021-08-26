# ContactPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | **String** | Proper email address. | 
**status** | Option<[**crate::models::ContactStatus**](ContactStatus.md)> | Status of the given resource | [optional]
**first_name** | Option<**String**> | First name. | [optional]
**last_name** | Option<**String**> | Last name. | [optional]
**custom_fields** | Option<**::std::collections::HashMap<String, String>**> | A key-value collection of custom contact fields which can be used in the system. Only already existing custom fields will be saved. | [optional]
**consent** | Option<[**crate::models::ConsentData**](ConsentData.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


