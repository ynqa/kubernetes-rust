# IoK8sApiCoreV1Volume

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**aws_elastic_block_store** | [***::models::IoK8sApiCoreV1AwsElasticBlockStoreVolumeSource**](io.k8s.api.core.v1.AWSElasticBlockStoreVolumeSource.md) |  | [optional] 
**azure_disk** | [***::models::IoK8sApiCoreV1AzureDiskVolumeSource**](io.k8s.api.core.v1.AzureDiskVolumeSource.md) |  | [optional] 
**azure_file** | [***::models::IoK8sApiCoreV1AzureFileVolumeSource**](io.k8s.api.core.v1.AzureFileVolumeSource.md) |  | [optional] 
**cephfs** | [***::models::IoK8sApiCoreV1CephFsVolumeSource**](io.k8s.api.core.v1.CephFSVolumeSource.md) |  | [optional] 
**cinder** | [***::models::IoK8sApiCoreV1CinderVolumeSource**](io.k8s.api.core.v1.CinderVolumeSource.md) |  | [optional] 
**config_map** | [***::models::IoK8sApiCoreV1ConfigMapVolumeSource**](io.k8s.api.core.v1.ConfigMapVolumeSource.md) |  | [optional] 
**downward_api** | [***::models::IoK8sApiCoreV1DownwardApiVolumeSource**](io.k8s.api.core.v1.DownwardAPIVolumeSource.md) |  | [optional] 
**empty_dir** | [***::models::IoK8sApiCoreV1EmptyDirVolumeSource**](io.k8s.api.core.v1.EmptyDirVolumeSource.md) |  | [optional] 
**fc** | [***::models::IoK8sApiCoreV1FcVolumeSource**](io.k8s.api.core.v1.FCVolumeSource.md) |  | [optional] 
**flex_volume** | [***::models::IoK8sApiCoreV1FlexVolumeSource**](io.k8s.api.core.v1.FlexVolumeSource.md) |  | [optional] 
**flocker** | [***::models::IoK8sApiCoreV1FlockerVolumeSource**](io.k8s.api.core.v1.FlockerVolumeSource.md) |  | [optional] 
**gce_persistent_disk** | [***::models::IoK8sApiCoreV1GcePersistentDiskVolumeSource**](io.k8s.api.core.v1.GCEPersistentDiskVolumeSource.md) |  | [optional] 
**git_repo** | [***::models::IoK8sApiCoreV1GitRepoVolumeSource**](io.k8s.api.core.v1.GitRepoVolumeSource.md) |  | [optional] 
**glusterfs** | [***::models::IoK8sApiCoreV1GlusterfsVolumeSource**](io.k8s.api.core.v1.GlusterfsVolumeSource.md) |  | [optional] 
**host_path** | [***::models::IoK8sApiCoreV1HostPathVolumeSource**](io.k8s.api.core.v1.HostPathVolumeSource.md) |  | [optional] 
**iscsi** | [***::models::IoK8sApiCoreV1IscsiVolumeSource**](io.k8s.api.core.v1.ISCSIVolumeSource.md) |  | [optional] 
**name** | **String** | Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names | 
**nfs** | [***::models::IoK8sApiCoreV1NfsVolumeSource**](io.k8s.api.core.v1.NFSVolumeSource.md) |  | [optional] 
**persistent_volume_claim** | [***::models::IoK8sApiCoreV1PersistentVolumeClaimVolumeSource**](io.k8s.api.core.v1.PersistentVolumeClaimVolumeSource.md) |  | [optional] 
**photon_persistent_disk** | [***::models::IoK8sApiCoreV1PhotonPersistentDiskVolumeSource**](io.k8s.api.core.v1.PhotonPersistentDiskVolumeSource.md) |  | [optional] 
**portworx_volume** | [***::models::IoK8sApiCoreV1PortworxVolumeSource**](io.k8s.api.core.v1.PortworxVolumeSource.md) |  | [optional] 
**projected** | [***::models::IoK8sApiCoreV1ProjectedVolumeSource**](io.k8s.api.core.v1.ProjectedVolumeSource.md) |  | [optional] 
**quobyte** | [***::models::IoK8sApiCoreV1QuobyteVolumeSource**](io.k8s.api.core.v1.QuobyteVolumeSource.md) |  | [optional] 
**rbd** | [***::models::IoK8sApiCoreV1RbdVolumeSource**](io.k8s.api.core.v1.RBDVolumeSource.md) |  | [optional] 
**scale_io** | [***::models::IoK8sApiCoreV1ScaleIoVolumeSource**](io.k8s.api.core.v1.ScaleIOVolumeSource.md) |  | [optional] 
**secret** | [***::models::IoK8sApiCoreV1SecretVolumeSource**](io.k8s.api.core.v1.SecretVolumeSource.md) |  | [optional] 
**storageos** | [***::models::IoK8sApiCoreV1StorageOsVolumeSource**](io.k8s.api.core.v1.StorageOSVolumeSource.md) |  | [optional] 
**vsphere_volume** | [***::models::IoK8sApiCoreV1VsphereVirtualDiskVolumeSource**](io.k8s.api.core.v1.VsphereVirtualDiskVolumeSource.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


