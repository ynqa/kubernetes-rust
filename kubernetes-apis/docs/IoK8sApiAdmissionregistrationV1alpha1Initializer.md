# IoK8sApiAdmissionregistrationV1alpha1Initializer

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Name is the identifier of the initializer. It will be added to the object that needs to be initialized. Name should be fully qualified, e.g., alwayspullimages.kubernetes.io, where \"alwayspullimages\" is the name of the webhook, and kubernetes.io is the name of the organization. Required | 
**rules** | [**Vec<::models::IoK8sApiAdmissionregistrationV1alpha1Rule>**](io.k8s.api.admissionregistration.v1alpha1.Rule.md) | Rules describes what resources/subresources the initializer cares about. The initializer cares about an operation if it matches _any_ Rule. Rule.Resources must not include subresources. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


