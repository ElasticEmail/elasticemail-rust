# Contact

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> | Proper email address. | [optional]
**status** | Option<[**models::ContactStatus**](ContactStatus.md)> |  | [optional]
**first_name** | Option<**String**> | First name. | [optional]
**last_name** | Option<**String**> | Last name. | [optional]
**custom_fields** | Option<**std::collections::HashMap<String, String>**> | A key-value collection of custom contact fields which can be used in the system. | [optional]
**consent** | Option<[**models::ConsentData**](ConsentData.md)> |  | [optional]
**source** | Option<[**models::ContactSource**](ContactSource.md)> |  | [optional]
**date_added** | Option<**String**> | Date of creation in YYYY-MM-DDThh:ii:ss format | [optional]
**date_updated** | Option<**String**> | Last change date | [optional]
**status_change_date** | Option<**String**> | Date of last status change. | [optional]
**activity** | Option<[**models::ContactActivity**](ContactActivity.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


