# IoK8sApiCoreV1DownwardApiVolumeFile

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**field_ref** | [***::models::IoK8sApiCoreV1ObjectFieldSelector**](io.k8s.api.core.v1.ObjectFieldSelector.md) |  | [optional] 
**mode** | **i32** | Optional: mode bits to use on this file, must be a value between 0 and 0777. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set. | [optional] 
**path** | **String** | Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..' | 
**resource_field_ref** | [***::models::IoK8sApiCoreV1ResourceFieldSelector**](io.k8s.api.core.v1.ResourceFieldSelector.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


