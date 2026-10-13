# Webhook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**webhook_id** | Option<**String**> | Public webhook ID | [optional]
**name** | Option<**String**> | Filename | [optional]
**date_created** | Option<**String**> | Creation date. | [optional]
**date_updated** | Option<**String**> | Last change date | [optional]
**url** | Option<**String**> | URL of notification. | [optional]
**notify_once_per_email** | Option<**bool**> |  | [optional]
**notification_for_sent** | Option<**bool**> |  | [optional]
**notification_for_opened** | Option<**bool**> |  | [optional]
**notification_for_clicked** | Option<**bool**> |  | [optional]
**notification_for_unsubscribed** | Option<**bool**> |  | [optional]
**notification_for_abuse_report** | Option<**bool**> |  | [optional]
**notification_for_error** | Option<**bool**> |  | [optional]
**is_enabled** | Option<**bool**> |  | [optional]
**signing_enabled** | Option<**bool**> | Requests carry an HMAC-SHA256 signature. Chosen when the webhook is created and can't be changed. | [optional]
**delivery_method** | Option<[**models::WebhookDeliveryMethod**](WebhookDeliveryMethod.md)> |  | [optional]
**signing_secret** | Option<**String**> | Signing secret - returned only once, when a webhook is created with signing or its secret is rotated. Store it safely. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


