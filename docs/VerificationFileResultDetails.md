# VerificationFileResultDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**verification_result** | Option<[**Vec<crate::models::EmailValidationResult>**](EmailValidationResult.md)> | Verification result's details | [optional]
**verification_id** | Option<**String**> | Identifier of this verification result | [optional]
**filename** | Option<**String**> | Origin file name | [optional]
**verification_status** | Option<[**crate::models::VerificationStatus**](VerificationStatus.md)> | In what state does this verification result currently is | [optional]
**file_upload_result** | Option<[**crate::models::FileUploadResult**](FileUploadResult.md)> | How many emails were detected in the file for verification | [optional]
**date_added** | Option<**String**> | Date of creation in YYYY-MM-DDThh:ii:ss format | [optional]
**source** | Option<**String**> | Origin file extension | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


