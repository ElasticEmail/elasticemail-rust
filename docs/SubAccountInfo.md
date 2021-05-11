# SubAccountInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**public_account_id** | Option<**String**> | Public key for limited access to your Account such as contact/add so you can use it safely on public websites. | [optional]
**email** | Option<**String**> | Proper email address. | [optional]
**settings** | Option<[**crate::models::SubaccountSettingsInfo**](SubaccountSettingsInfo.md)> | SubAccount settings | [optional]
**last_activity** | Option<**String**> | Date of last activity on Account | [optional]
**email_credits** | Option<**i32**> | Amount of email credits | [optional]
**total_emails_sent** | Option<**i64**> | Amount of emails sent from this Account | [optional]
**reputation** | Option<**f64**> | Numeric reputation | [optional]
**status** | Option<[**crate::models::AccountStatusEnum**](AccountStatusEnum.md)> | Account's current status. | [optional]
**contacts_count** | Option<**i32**> | How many contacts this SubAccount has stored | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


