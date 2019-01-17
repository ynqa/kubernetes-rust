# IoK8sApiSettingsV1alpha1PodPresetSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**env** | [**Vec<::models::IoK8sApiCoreV1EnvVar>**](io.k8s.api.core.v1.EnvVar.md) | Env defines the collection of EnvVar to inject into containers. | [optional] 
**env_from** | [**Vec<::models::IoK8sApiCoreV1EnvFromSource>**](io.k8s.api.core.v1.EnvFromSource.md) | EnvFrom defines the collection of EnvFromSource to inject into containers. | [optional] 
**selector** | [***::models::IoK8sApimachineryPkgApisMetaV1LabelSelector**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector.md) |  | [optional] 
**volume_mounts** | [**Vec<::models::IoK8sApiCoreV1VolumeMount>**](io.k8s.api.core.v1.VolumeMount.md) | VolumeMounts defines the collection of VolumeMount to inject into containers. | [optional] 
**volumes** | [**Vec<::models::IoK8sApiCoreV1Volume>**](io.k8s.api.core.v1.Volume.md) | Volumes defines the collection of Volume to inject into the pod. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


