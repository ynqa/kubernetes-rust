# IoK8sApiCoreV1PersistentVolumeClaimStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_modes** | **Vec<String>** | AccessModes contains the actual access modes the volume backing the PVC has. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1 | [optional] 
**capacity** | **::std::collections::HashMap<String, String>** | Represents the actual resources of the underlying volume. | [optional] 
**conditions** | [**Vec<::models::IoK8sApiCoreV1PersistentVolumeClaimCondition>**](io.k8s.api.core.v1.PersistentVolumeClaimCondition.md) | Current Condition of persistent volume claim. If underlying persistent volume is being resized then the Condition will be set to 'ResizeStarted'. | [optional] 
**phase** | **String** | Phase represents the current phase of PersistentVolumeClaim. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


