# EmailJobStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | ID number of your attachment | [optional]
**status** | Option<**String**> | Name of status: submitted, complete, in_progress | [optional]
**recipients_count** | Option<**i32**> |  | [optional]
**failed** | Option<[**Vec<models::EmailJobFailedStatus>**](EmailJobFailedStatus.md)> |  | [optional]
**failed_count** | Option<**i32**> | Total emails failed. | [optional]
**sent** | Option<**Vec<String>**> |  | [optional]
**sent_count** | Option<**i32**> | Total emails sent. | [optional]
**delivered** | Option<**Vec<String>**> | Number of delivered messages | [optional]
**delivered_count** | Option<**i32**> |  | [optional]
**pending** | Option<**Vec<String>**> |  | [optional]
**pending_count** | Option<**i32**> |  | [optional]
**opened** | Option<**Vec<String>**> | Number of opened messages | [optional]
**opened_count** | Option<**i32**> | Total emails opened. | [optional]
**clicked** | Option<**Vec<String>**> | Number of clicked messages | [optional]
**clicked_count** | Option<**i32**> | Total emails clicked | [optional]
**unsubscribed** | Option<**Vec<String>**> | Number of unsubscribed messages | [optional]
**unsubscribed_count** | Option<**i32**> | Total emails unsubscribed | [optional]
**abuse_reports** | Option<**Vec<String>**> |  | [optional]
**abuse_reports_count** | Option<**i32**> |  | [optional]
**message_ids** | Option<**Vec<String>**> | List of all MessageIDs for this job. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


