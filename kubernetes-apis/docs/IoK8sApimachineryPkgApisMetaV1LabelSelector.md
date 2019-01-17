# IoK8sApimachineryPkgApisMetaV1LabelSelector

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**match_expressions** | [**Vec<::models::IoK8sApimachineryPkgApisMetaV1LabelSelectorRequirement>**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelectorRequirement.md) | matchExpressions is a list of label selector requirements. The requirements are ANDed. | [optional] 
**match_labels** | **::std::collections::HashMap<String, String>** | matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is \"key\", the operator is \"In\", and the values array contains only \"value\". The requirements are ANDed. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


