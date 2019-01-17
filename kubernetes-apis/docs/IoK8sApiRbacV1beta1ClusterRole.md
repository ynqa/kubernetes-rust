# IoK8sApiRbacV1beta1ClusterRole

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aggregation_rule** | [***::models::IoK8sApiRbacV1beta1AggregationRule**](io.k8s.api.rbac.v1beta1.AggregationRule.md) |  | [optional] 
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] 
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] 
**metadata** | [***::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta**](io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.md) |  | [optional] 
**rules** | [**Vec<::models::IoK8sApiRbacV1beta1PolicyRule>**](io.k8s.api.rbac.v1beta1.PolicyRule.md) | Rules holds all the PolicyRules for this ClusterRole | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


