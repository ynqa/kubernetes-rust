# IoK8sApiAppsV1beta2DaemonSetSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_ready_seconds** | **i32** | The minimum number of seconds for which a newly created DaemonSet pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready). | [optional] 
**revision_history_limit** | **i32** | The number of old history to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. Defaults to 10. | [optional] 
**selector** | [***::models::IoK8sApimachineryPkgApisMetaV1LabelSelector**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector.md) |  | 
**template** | [***::models::IoK8sApiCoreV1PodTemplateSpec**](io.k8s.api.core.v1.PodTemplateSpec.md) |  | 
**update_strategy** | [***::models::IoK8sApiAppsV1beta2DaemonSetUpdateStrategy**](io.k8s.api.apps.v1beta2.DaemonSetUpdateStrategy.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


