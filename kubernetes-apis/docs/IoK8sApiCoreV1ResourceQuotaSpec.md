# IoK8sApiCoreV1ResourceQuotaSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**hard** | **::std::collections::HashMap<String, String>** | hard is the set of desired hard limits for each named resource. More info: https://kubernetes.io/docs/concepts/policy/resource-quotas/ | [optional] 
**scope_selector** | [***::models::IoK8sApiCoreV1ScopeSelector**](io.k8s.api.core.v1.ScopeSelector.md) |  | [optional] 
**scopes** | **Vec<String>** | A collection of filters that must match each object tracked by a quota. If not specified, the quota matches all objects. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


