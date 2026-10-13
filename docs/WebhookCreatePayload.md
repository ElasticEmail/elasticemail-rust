# WebhookCreatePayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Filename | 
**url** | **String** | URL of notification. | 
**notify_once_per_email** | Option<**bool**> |  | [optional]
**notification_for_sent** | Option<**bool**> |  | [optional]
**notification_for_opened** | Option<**bool**> |  | [optional]
**notification_for_clicked** | Option<**bool**> |  | [optional]
**notification_for_unsubscribed** | Option<**bool**> |  | [optional]
**notification_for_abuse_report** | Option<**bool**> |  | [optional]
**notification_for_error** | Option<**bool**> |  | [optional]
**enable_signing** | Option<**bool**> | Sign every request with HMAC-SHA256. Can be set only when the webhook is created. Default: false. | [optional]
**delivery_method** | Option<[**models::WebhookDeliveryMethod**](WebhookDeliveryMethod.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


