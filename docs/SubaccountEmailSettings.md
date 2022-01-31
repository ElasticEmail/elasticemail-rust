# SubaccountEmailSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**monthly_refill_credits** | Option<**i32**> | Amount of credits added to Account automatically | [optional]
**requires_email_credits** | Option<**bool**> | True, if Account needs credits to send emails. Otherwise, false | [optional]
**email_size_limit** | Option<**i32**> | Maximum size of email including attachments in MB's | [optional]
**daily_send_limit** | Option<**i32**> | Amount of emails Account can send daily | [optional]
**max_contacts** | Option<**i32**> | Maximum number of contacts the Account can have. 0 means that parent account's limit is used. | [optional]
**enable_private_ip_purchase** | Option<**bool**> | Can the SubAccount purchase Private IP for themselves | [optional]
**pool_name** | Option<**String**> | Name of your custom IP Pool to be used in the sending process | [optional]
**valid_sender_domain_only** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


