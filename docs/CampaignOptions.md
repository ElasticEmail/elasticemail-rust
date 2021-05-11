# CampaignOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delivery_optimization** | Option<[**crate::models::DeliveryOptimizationType**](DeliveryOptimizationType.md)> | How to order email delivery - by recipients' engagement score or by the time they open the most of the emails that were sent to them | [optional]
**track_opens** | Option<**bool**> | Should the opens be tracked? If no value has been provided, Account's default setting will be used. | [optional]
**track_clicks** | Option<**bool**> | Should the clicks be tracked? If no value has been provided, Account's default setting will be used. | [optional]
**schedule_for** | Option<**String**> | Date when this Campaign is scheduled to be sent on | [optional]
**split_options** | Option<[**crate::models::SplitOptions**](SplitOptions.md)> | Optional options for A/X split campaigns. Will be ignored if only one template content was provided | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


