# IoK8sApimachineryPkgApisMetaV1ApiResource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**categories** | **Vec<String>** | categories is a list of the grouped resources this resource belongs to (e.g. 'all') | [optional] 
**group** | **String** | group is the preferred group of the resource.  Empty implies the group of the containing resource list. For subresources, this may have a different value, for example: Scale\". | [optional] 
**kind** | **String** | kind is the kind for the resource (e.g. 'Foo' is the kind for a resource 'foo') | 
**name** | **String** | name is the plural name of the resource. | 
**namespaced** | **bool** | namespaced indicates if a resource is namespaced or not. | 
**short_names** | **Vec<String>** | shortNames is a list of suggested short names of the resource. | [optional] 
**singular_name** | **String** | singularName is the singular name of the resource.  This allows clients to handle plural and singular opaquely. The singularName is more correct for reporting status on a single item and both singular and plural are allowed from the kubectl CLI interface. | 
**verbs** | **Vec<String>** | verbs is a list of supported kube verbs (this includes get, list, watch, create, update, patch, delete, deletecollection, and proxy) | 
**version** | **String** | version is the preferred version of the resource.  Empty implies the version of the containing resource list For subresources, this may have a different value, for example: v1 (while inside a v1beta1 version of the core resource's group)\". | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


