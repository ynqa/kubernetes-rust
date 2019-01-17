# IoK8sApiExtensionsV1beta1DeploymentSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**min_ready_seconds** | **i32** | Minimum number of seconds for which a newly created pod should be ready without any of its container crashing, for it to be considered available. Defaults to 0 (pod will be considered available as soon as it is ready) | [optional] 
**paused** | **bool** | Indicates that the deployment is paused and will not be processed by the deployment controller. | [optional] 
**progress_deadline_seconds** | **i32** | The maximum time in seconds for a deployment to make progress before it is considered to be failed. The deployment controller will continue to process failed deployments and a condition with a ProgressDeadlineExceeded reason will be surfaced in the deployment status. Note that progress will not be estimated during the time a deployment is paused. This is set to the max value of int32 (i.e. 2147483647) by default, which means \"no deadline\". | [optional] 
**replicas** | **i32** | Number of desired pods. This is a pointer to distinguish between explicit zero and not specified. Defaults to 1. | [optional] 
**revision_history_limit** | **i32** | The number of old ReplicaSets to retain to allow rollback. This is a pointer to distinguish between explicit zero and not specified. This is set to the max value of int32 (i.e. 2147483647) by default, which means \"retaining all old RelicaSets\". | [optional] 
**rollback_to** | [***::models::IoK8sApiExtensionsV1beta1RollbackConfig**](io.k8s.api.extensions.v1beta1.RollbackConfig.md) |  | [optional] 
**selector** | [***::models::IoK8sApimachineryPkgApisMetaV1LabelSelector**](io.k8s.apimachinery.pkg.apis.meta.v1.LabelSelector.md) |  | [optional] 
**strategy** | [***::models::IoK8sApiExtensionsV1beta1DeploymentStrategy**](io.k8s.api.extensions.v1beta1.DeploymentStrategy.md) |  | [optional] 
**template** | [***::models::IoK8sApiCoreV1PodTemplateSpec**](io.k8s.api.core.v1.PodTemplateSpec.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


