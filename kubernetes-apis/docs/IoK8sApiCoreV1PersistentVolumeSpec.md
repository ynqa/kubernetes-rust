# IoK8sApiCoreV1PersistentVolumeSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_modes** | **Vec<String>** | AccessModes contains all ways the volume can be mounted. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes | [optional] 
**aws_elastic_block_store** | [***::models::IoK8sApiCoreV1AwsElasticBlockStoreVolumeSource**](io.k8s.api.core.v1.AWSElasticBlockStoreVolumeSource.md) |  | [optional] 
**azure_disk** | [***::models::IoK8sApiCoreV1AzureDiskVolumeSource**](io.k8s.api.core.v1.AzureDiskVolumeSource.md) |  | [optional] 
**azure_file** | [***::models::IoK8sApiCoreV1AzureFilePersistentVolumeSource**](io.k8s.api.core.v1.AzureFilePersistentVolumeSource.md) |  | [optional] 
**capacity** | **::std::collections::HashMap<String, String>** | A description of the persistent volume's resources and capacity. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity | [optional] 
**cephfs** | [***::models::IoK8sApiCoreV1CephFsPersistentVolumeSource**](io.k8s.api.core.v1.CephFSPersistentVolumeSource.md) |  | [optional] 
**cinder** | [***::models::IoK8sApiCoreV1CinderPersistentVolumeSource**](io.k8s.api.core.v1.CinderPersistentVolumeSource.md) |  | [optional] 
**claim_ref** | [***::models::IoK8sApiCoreV1ObjectReference**](io.k8s.api.core.v1.ObjectReference.md) |  | [optional] 
**csi** | [***::models::IoK8sApiCoreV1CsiPersistentVolumeSource**](io.k8s.api.core.v1.CSIPersistentVolumeSource.md) |  | [optional] 
**fc** | [***::models::IoK8sApiCoreV1FcVolumeSource**](io.k8s.api.core.v1.FCVolumeSource.md) |  | [optional] 
**flex_volume** | [***::models::IoK8sApiCoreV1FlexPersistentVolumeSource**](io.k8s.api.core.v1.FlexPersistentVolumeSource.md) |  | [optional] 
**flocker** | [***::models::IoK8sApiCoreV1FlockerVolumeSource**](io.k8s.api.core.v1.FlockerVolumeSource.md) |  | [optional] 
**gce_persistent_disk** | [***::models::IoK8sApiCoreV1GcePersistentDiskVolumeSource**](io.k8s.api.core.v1.GCEPersistentDiskVolumeSource.md) |  | [optional] 
**glusterfs** | [***::models::IoK8sApiCoreV1GlusterfsPersistentVolumeSource**](io.k8s.api.core.v1.GlusterfsPersistentVolumeSource.md) |  | [optional] 
**host_path** | [***::models::IoK8sApiCoreV1HostPathVolumeSource**](io.k8s.api.core.v1.HostPathVolumeSource.md) |  | [optional] 
**iscsi** | [***::models::IoK8sApiCoreV1IscsiPersistentVolumeSource**](io.k8s.api.core.v1.ISCSIPersistentVolumeSource.md) |  | [optional] 
**local** | [***::models::IoK8sApiCoreV1LocalVolumeSource**](io.k8s.api.core.v1.LocalVolumeSource.md) |  | [optional] 
**mount_options** | **Vec<String>** | A list of mount options, e.g. [\"ro\", \"soft\"]. Not validated - mount will simply fail if one is invalid. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes/#mount-options | [optional] 
**nfs** | [***::models::IoK8sApiCoreV1NfsVolumeSource**](io.k8s.api.core.v1.NFSVolumeSource.md) |  | [optional] 
**node_affinity** | [***::models::IoK8sApiCoreV1VolumeNodeAffinity**](io.k8s.api.core.v1.VolumeNodeAffinity.md) |  | [optional] 
**persistent_volume_reclaim_policy** | **String** | What happens to a persistent volume when released from its claim. Valid options are Retain (default for manually created PersistentVolumes), Delete (default for dynamically provisioned PersistentVolumes), and Recycle (deprecated). Recycle must be supported by the volume plugin underlying this PersistentVolume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#reclaiming | [optional] 
**photon_persistent_disk** | [***::models::IoK8sApiCoreV1PhotonPersistentDiskVolumeSource**](io.k8s.api.core.v1.PhotonPersistentDiskVolumeSource.md) |  | [optional] 
**portworx_volume** | [***::models::IoK8sApiCoreV1PortworxVolumeSource**](io.k8s.api.core.v1.PortworxVolumeSource.md) |  | [optional] 
**quobyte** | [***::models::IoK8sApiCoreV1QuobyteVolumeSource**](io.k8s.api.core.v1.QuobyteVolumeSource.md) |  | [optional] 
**rbd** | [***::models::IoK8sApiCoreV1RbdPersistentVolumeSource**](io.k8s.api.core.v1.RBDPersistentVolumeSource.md) |  | [optional] 
**scale_io** | [***::models::IoK8sApiCoreV1ScaleIoPersistentVolumeSource**](io.k8s.api.core.v1.ScaleIOPersistentVolumeSource.md) |  | [optional] 
**storage_class_name** | **String** | Name of StorageClass to which this persistent volume belongs. Empty value means that this volume does not belong to any StorageClass. | [optional] 
**storageos** | [***::models::IoK8sApiCoreV1StorageOsPersistentVolumeSource**](io.k8s.api.core.v1.StorageOSPersistentVolumeSource.md) |  | [optional] 
**volume_mode** | **String** | volumeMode defines if a volume is intended to be used with a formatted filesystem or to remain in raw block state. Value of Filesystem is implied when not included in spec. This is a beta feature. | [optional] 
**vsphere_volume** | [***::models::IoK8sApiCoreV1VsphereVirtualDiskVolumeSource**](io.k8s.api.core.v1.VsphereVirtualDiskVolumeSource.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


