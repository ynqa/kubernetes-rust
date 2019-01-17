# IoK8sApiAdmissionregistrationV1alpha1Rule

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_groups** | **Vec<String>** | APIGroups is the API groups the resources belong to. '*' is all groups. If '*' is present, the length of the slice must be one. Required. | [optional] 
**api_versions** | **Vec<String>** | APIVersions is the API versions the resources belong to. '*' is all versions. If '*' is present, the length of the slice must be one. Required. | [optional] 
**resources** | **Vec<String>** | Resources is a list of resources this rule applies to.  For example: 'pods' means pods. 'pods/log' means the log subresource of pods. '*' means all resources, but not subresources. 'pods/_*' means all subresources of pods. '*_/scale' means all scale subresources. '*_/_*' means all resources and their subresources.  If wildcard is present, the validation rule will ensure resources do not overlap with each other.  Depending on the enclosing object, subresources might not be allowed. Required. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


