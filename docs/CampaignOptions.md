# CampaignOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**delivery_optimization** | Option<[**models::DeliveryOptimizationType**](DeliveryOptimizationType.md)> |  | [optional]
**track_opens** | Option<**bool**> | Should the opens be tracked? If no value has been provided, Account's default setting will be used. | [optional]
**track_clicks** | Option<**bool**> | Should the clicks be tracked? If no value has been provided, Account's default setting will be used. | [optional]
**schedule_for** | Option<**String**> | Date when this Campaign is scheduled to be sent on | [optional]
**trigger_frequency** | Option<**f64**> | How often (in minutes) to send the campaign | [optional]
**trigger_count** | Option<**i32**> | How many times send the campaign | [optional]
**split_options** | Option<[**models::SplitOptions**](SplitOptions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


