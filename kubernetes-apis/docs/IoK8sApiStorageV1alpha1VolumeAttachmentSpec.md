# IoK8sApiStorageV1alpha1VolumeAttachmentSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**attacher** | **String** | Attacher indicates the name of the volume driver that MUST handle this request. This is the name returned by GetPluginName(). | 
**node_name** | **String** | The node that the volume should be attached to. | 
**source** | [***::models::IoK8sApiStorageV1alpha1VolumeAttachmentSource**](io.k8s.api.storage.v1alpha1.VolumeAttachmentSource.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


