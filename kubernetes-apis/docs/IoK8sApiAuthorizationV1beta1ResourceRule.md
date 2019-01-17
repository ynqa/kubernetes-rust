# IoK8sApiAuthorizationV1beta1ResourceRule

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_groups** | **Vec<String>** | APIGroups is the name of the APIGroup that contains the resources.  If multiple API groups are specified, any action requested against one of the enumerated resources in any API group will be allowed.  \"*\" means all. | [optional] 
**resource_names** | **Vec<String>** | ResourceNames is an optional white list of names that the rule applies to.  An empty set means that everything is allowed.  \"*\" means all. | [optional] 
**resources** | **Vec<String>** | Resources is a list of resources this rule applies to.  \"*\" means all in the specified apiGroups.  \"*_/foo\" represents the subresource 'foo' for all resources in the specified apiGroups. | [optional] 
**verbs** | **Vec<String>** | Verb is a list of kubernetes resource API verbs, like: get, list, watch, create, update, delete, proxy.  \"*\" means all. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


