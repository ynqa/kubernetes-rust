# IoK8sApiextensionsApiserverPkgApisApiextensionsV1beta1CustomResourceDefinitionNames

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**categories** | **Vec<String>** | Categories is a list of grouped resources custom resources belong to (e.g. 'all') | [optional] 
**kind** | **String** | Kind is the serialized kind of the resource.  It is normally CamelCase and singular. | 
**list_kind** | **String** | ListKind is the serialized kind of the list for this resource.  Defaults to <kind>List. | [optional] 
**plural** | **String** | Plural is the plural name of the resource to serve.  It must match the name of the CustomResourceDefinition-registration too: plural.group and it must be all lowercase. | 
**short_names** | **Vec<String>** | ShortNames are short names for the resource.  It must be all lowercase. | [optional] 
**singular** | **String** | Singular is the singular name of the resource.  It must be all lowercase  Defaults to lowercased <kind> | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


