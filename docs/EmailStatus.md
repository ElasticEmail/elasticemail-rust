# EmailStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from** | Option<**String**> | Email address this email was sent from. | [optional]
**to** | Option<**String**> | Email address this email was sent to. | [optional]
**date** | Option<**String**> | Date the email was submitted. | [optional]
**status** | Option<[**crate::models::LogJobStatus**](LogJobStatus.md)> |  | [optional]
**status_name** | Option<**String**> | Name of email's status | [optional]
**status_change_date** | Option<**String**> | Date of last status change. | [optional]
**date_sent** | Option<**String**> | Date when the email was sent | [optional]
**date_opened** | Option<**String**> | Date when the email changed the status to 'opened' | [optional]
**date_clicked** | Option<**String**> | Date when the email changed the status to 'clicked' | [optional]
**error_message** | Option<**String**> | Detailed error or bounced message. | [optional]
**transaction_id** | Option<**String**> | ID number of transaction | [optional]
**envelope_from** | Option<**String**> | Envelope from address | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


