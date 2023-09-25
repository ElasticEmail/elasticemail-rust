# EmailValidationResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account** | Option<**String**> | Local part of an email | [optional]
**domain** | Option<**String**> | Name of selected domain. | [optional]
**email** | Option<**String**> | Full email address that was verified | [optional]
**suggested_spelling** | Option<**String**> | Suggested spelling if a possible mistake was found | [optional]
**disposable** | Option<**bool**> | Does the email have a temporary domain | [optional]
**role** | Option<**bool**> | Is an email a role email (e.g. info@, noreply@ etc.) | [optional]
**reason** | Option<**String**> | All detected issues | [optional]
**date_added** | Option<**String**> | Date of creation in YYYY-MM-DDThh:ii:ss format | [optional]
**result** | Option<[**crate::models::EmailValidationStatus**](EmailValidationStatus.md)> |  | [optional]
**predicted_score** | Option<**f32**> |  | [optional]
**predicted_status** | Option<[**crate::models::EmailPredictedValidationStatus**](EmailPredictedValidationStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


