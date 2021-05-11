# RecipientEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**transaction_id** | Option<**String**> | ID number of transaction | [optional]
**msg_id** | Option<**String**> | ID number of selected message. | [optional]
**from_email** | Option<**String**> | Default From: email address. | [optional]
**to** | Option<**String**> | Ending date for search in YYYY-MM-DDThh:mm:ss format. | [optional]
**subject** | Option<**String**> | Default subject of email. | [optional]
**event_type** | Option<[**crate::models::EventType**](EventType.md)> | Type of an Event | [optional]
**event_date** | Option<**String**> | Creation date | [optional]
**channel_name** | Option<**String**> | Name of selected channel. | [optional]
**message_category** | Option<[**crate::models::MessageCategory**](MessageCategory.md)> | Message category | [optional]
**next_try_on** | Option<**String**> | Date of next try | [optional]
**message** | Option<**String**> | Content of message, HTML encoded | [optional]
**ip_address** | Option<**String**> | IP which this email was sent through | [optional]
**pool_name** | Option<**String**> | Name of an IP pool this email was sent through | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


