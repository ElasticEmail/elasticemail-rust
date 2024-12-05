# DomainData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**validation_log** | Option<**String**> | Domain validation results - when domain has been running through validation process | [optional]
**domain** | Option<**String**> | Name of selected domain. | [optional]
**default_domain** | Option<**bool**> | True, if domain is used as default. Otherwise, false, | [optional]
**spf** | Option<**bool**> | True, if SPF record is verified | [optional]
**dkim** | Option<**bool**> | True, if DKIM record is verified | [optional]
**mx** | Option<**bool**> | True, if MX record is verified | [optional]
**dmarc** | Option<**bool**> |  | [optional]
**is_rewrite_domain_valid** | Option<**bool**> | True, if tracking CNAME record is verified | [optional]
**verify** | Option<**bool**> | True, if DKIM, SPF, or tracking are still to be verified | [optional]
**r#type** | Option<[**models::TrackingType**](TrackingType.md)> |  | [optional]
**tracking_status** | Option<[**models::TrackingValidationStatus**](TrackingValidationStatus.md)> |  | [optional]
**certificate_status** | Option<[**models::CertificateValidationStatus**](CertificateValidationStatus.md)> |  | [optional]
**certificate_validation_error** | Option<**String**> |  | [optional]
**tracking_type_user_request** | Option<[**models::TrackingType**](TrackingType.md)> |  | [optional]
**verp** | Option<**bool**> |  | [optional]
**custom_bounces_domain** | Option<**String**> |  | [optional]
**is_custom_bounces_domain_default** | Option<**bool**> |  | [optional]
**is_marked_for_deletion** | Option<**bool**> |  | [optional]
**ownership** | Option<[**models::DomainOwner**](DomainOwner.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


