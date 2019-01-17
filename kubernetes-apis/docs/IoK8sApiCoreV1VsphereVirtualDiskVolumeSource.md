# IoK8sApiCoreV1VsphereVirtualDiskVolumeSource

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fs_type** | **String** | Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. \"ext4\", \"xfs\", \"ntfs\". Implicitly inferred to be \"ext4\" if unspecified. | [optional] 
**storage_policy_id** | **String** | Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName. | [optional] 
**storage_policy_name** | **String** | Storage Policy Based Management (SPBM) profile name. | [optional] 
**volume_path** | **String** | Path that identifies vSphere volume vmdk | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


