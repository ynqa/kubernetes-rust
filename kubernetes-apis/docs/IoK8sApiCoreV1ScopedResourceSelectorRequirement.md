# IoK8sApiCoreV1ScopedResourceSelectorRequirement

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operator** | **String** | Represents a scope's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. | 
**scope_name** | **String** | The name of the scope that the selector applies to. | 
**values** | **Vec<String>** | An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


