# IoK8sApiCoreV1ContainerStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_id** | **String** | Container's ID in the format 'docker://<container_id>'. | [optional] 
**image** | **String** | The image the container is running. More info: https://kubernetes.io/docs/concepts/containers/images | 
**image_id** | **String** | ImageID of the container's image. | 
**last_state** | [***::models::IoK8sApiCoreV1ContainerState**](io.k8s.api.core.v1.ContainerState.md) |  | [optional] 
**name** | **String** | This must be a DNS_LABEL. Each container in a pod must have a unique name. Cannot be updated. | 
**ready** | **bool** | Specifies whether the container has passed its readiness probe. | 
**restart_count** | **i32** | The number of times the container has been restarted, currently based on the number of dead containers that have not yet been removed. Note that this is calculated from dead containers. But those containers are subject to garbage collection. This value will get capped at 5 by GC. | 
**state** | [***::models::IoK8sApiCoreV1ContainerState**](io.k8s.api.core.v1.ContainerState.md) |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


