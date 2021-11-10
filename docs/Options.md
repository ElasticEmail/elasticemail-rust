# Options

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**time_offset** | Option<**i32**> | By how long should an e-mail be delayed (in minutes). Maximum is 35 days. | [optional]
**pool_name** | Option<**String**> | Name of your custom IP Pool to be used in the sending process | [optional]
**channel_name** | Option<**String**> | Name of selected channel. | [optional]
**encoding** | Option<[**crate::models::EncodingType**](EncodingType.md)> |  | [optional]
**track_opens** | Option<**bool**> | Should the opens be tracked? If no value has been provided, Account's default setting will be used. | [optional]
**track_clicks** | Option<**bool**> | Should the clicks be tracked? If no value has been provided, Account's default setting will be used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


