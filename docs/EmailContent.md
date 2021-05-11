# EmailContent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**body** | Option<[**Vec<crate::models::BodyPart>**](BodyPart.md)> | List of e-mail body parts, with user-provided MIME types (text/html, text/plain etc) | [optional]
**merge** | Option<**::std::collections::HashMap<String, String>**> | A key-value collection of custom merge fields, shared between recipients. Should be used in e-mail body like so: {firstname}, {lastname} etc. | [optional]
**attachments** | Option<[**Vec<crate::models::MessageAttachment>**](MessageAttachment.md)> | Attachments provided by sending binary data | [optional]
**headers** | Option<**::std::collections::HashMap<String, String>**> | A key-value collection of custom e-mail headers. | [optional]
**postback** | Option<**String**> | Postback header. | [optional]
**from** | Option<**String**> | Your e-mail with an optional name (e.g.: John Doe <email@domain.com>) | [optional]
**reply_to** | Option<**String**> | To what address should the recipients reply to (e.g. John Doe <email@domain.com>) | [optional]
**subject** | Option<**String**> | Default subject of email. | [optional]
**template_name** | Option<**String**> | Name of template. | [optional]
**attach_files** | Option<**Vec<String>**> | Names of previously uploaded files that should be sent as downloadable attachments | [optional]
**utm** | Option<[**crate::models::Utm**](Utm.md)> | Utm marketing data to be attached to every link in this e-mail. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


