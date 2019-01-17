# IoK8sApiCoreV1PersistentVolumeClaimSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_modes** | **Vec<String>** | AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1 | [optional] 
**data_source** | [***::models::IoK8sApiCoreV1TypedLocalObjectReference**](io.k8s.api.core.v1.TypedLocalObjectReference.md) |  | [optional] 
**resources** | [***::models::IoK8sApiCoreV1ResourceRequirements**](io.k8s.api.core.v1.ResourceRequirements.md) |  | [optional] 
**selector** | [***::models::IoK8sApimachineryPkgApisMetaV1LabelSelector**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector.md) |  | [optional] 
**storage_class_name** | **String** | Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1 | [optional] 
**volume_mode** | **String** | volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec. This is a beta feature. | [optional] 
**volume_name** | **String** | VolumeName is the binding reference to the PersistentVolume backing this claim. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


