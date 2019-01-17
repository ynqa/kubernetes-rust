# IoK8sApiStorageV1StorageClass

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_volume_expansion** | **bool** | AllowVolumeExpansion shows whether the storage class allow volume expand | [optional] 
**allowed_topologies** | [**Vec<::models::IoK8sApiCoreV1TopologySelectorTerm>**](io.k8s.api.core.v1.TopologySelectorTerm.md) | Restrict the node topologies where volumes can be dynamically provisioned. Each volume plugin defines its own supported topology specifications. An empty TopologySelectorTerm list means there is no topology restriction. This field is only honored by servers that enable the VolumeScheduling feature. | [optional] 
**api_version** | **String** | APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources | [optional] 
**kind** | **String** | Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds | [optional] 
**metadata** | [***::models::IoK8sApimachineryPkgApisMetaV1ObjectMeta**](io.k8s.apimachinery.pkg.apis.meta.v1.ObjectMeta.md) |  | [optional] 
**mount_options** | **Vec<String>** | Dynamically provisioned PersistentVolumes of this storage class are created with these mountOptions, e.g. [\"ro\", \"soft\"]. Not validated - mount of the PVs will simply fail if one is invalid. | [optional] 
**parameters** | **::std::collections::HashMap<String, String>** | Parameters holds the parameters for the provisioner that should create volumes of this storage class. | [optional] 
**provisioner** | **String** | Provisioner indicates the type of the provisioner. | 
**reclaim_policy** | **String** | Dynamically provisioned PersistentVolumes of this storage class are created with this reclaimPolicy. Defaults to Delete. | [optional] 
**volume_binding_mode** | **String** | VolumeBindingMode indicates how PersistentVolumeClaims should be provisioned and bound.  When unset, VolumeBindingImmediate is used. This field is only honored by servers that enable the VolumeScheduling feature. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


