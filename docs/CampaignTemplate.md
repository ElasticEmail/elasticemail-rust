# CampaignTemplate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**poolname** | Option<**String**> | Name of your custom IP Pool to be used in the sending process | [optional]
**from** | **String** | Your e-mail with an optional name (e.g.: John Doe <email@domain.com>) | 
**reply_to** | Option<**String**> | To what address should the recipients reply to (e.g. John Doe <email@domain.com>) | [optional]
**subject** | Option<**String**> | Default subject of email. | [optional]
**template_name** | Option<**String**> | Name of template. | [optional]
**attach_files** | Option<**Vec<String>**> | Names of previously uploaded files that should be sent as downloadable attachments | [optional]
**utm** | Option<[**models::Utm**](Utm.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


