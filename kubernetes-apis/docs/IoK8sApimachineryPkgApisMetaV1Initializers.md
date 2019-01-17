# IoK8sApimachineryPkgApisMetaV1Initializers

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**pending** | [**Vec<::models::IoK8sApimachineryPkgApisMetaV1Initializer>**](io.k8s.apimachinery.pkg.apis.meta.v1.Initializer.md) | Pending is a list of initializers that must execute in order before this object is visible. When the last pending initializer is removed, and no failing result is set, the initializers struct will be set to nil and the object is considered as initialized and visible to all clients. | 
**result** | [***::models::IoK8sApimachineryPkgApisMetaV1Status**](io.k8s.apimachinery.pkg.apis.meta.v1.Status.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


