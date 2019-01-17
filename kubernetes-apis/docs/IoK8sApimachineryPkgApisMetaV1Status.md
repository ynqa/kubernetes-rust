# IoK8sApimachineryPkgApisMetaV1Status

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] 
**code** | **i32** | Suggested HTTP return code for this status, 0 if not set. | [optional] 
**details** | [***::models::IoK8sApimachineryPkgApisMetaV1StatusDetails**](io.k8s.apimachinery.pkg.apis.meta.v1.StatusDetails.md) |  | [optional] 
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] 
**message** | **String** | A human-readable description of the status of this operation. | [optional] 
**metadata** | [***::models::IoK8sApimachineryPkgApisMetaV1ListMeta**](io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta.md) |  | [optional] 
**reason** | **String** | A machine-readable description of why this operation is in the \"Failure\" status. If this value is empty there is no information available. A Reason clarifies an HTTP status code but does not override it. | [optional] 
**status** | **String** | Status of the operation. One of: \"Success\" or \"Failure\". More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


