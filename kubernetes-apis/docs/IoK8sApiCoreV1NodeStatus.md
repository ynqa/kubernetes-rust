# IoK8sApiCoreV1NodeStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**addresses** | [**Vec<::models::IoK8sApiCoreV1NodeAddress>**](io.k8s.api.core.v1.NodeAddress.md) | List of addresses reachable to the node. Queried from cloud provider, if available. More info: https://kubernetes.io/docs/concepts/nodes/node/#addresses | [optional] 
**allocatable** | **::std::collections::HashMap<String, String>** | Allocatable represents the resources of a node that are available for scheduling. Defaults to Capacity. | [optional] 
**capacity** | **::std::collections::HashMap<String, String>** | Capacity represents the total resources of a node. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#capacity | [optional] 
**conditions** | [**Vec<::models::IoK8sApiCoreV1NodeCondition>**](io.k8s.api.core.v1.NodeCondition.md) | Conditions is an array of current observed node conditions. More info: https://kubernetes.io/docs/concepts/nodes/node/#condition | [optional] 
**config** | [***::models::IoK8sApiCoreV1NodeConfigStatus**](io.k8s.api.core.v1.NodeConfigStatus.md) |  | [optional] 
**daemon_endpoints** | [***::models::IoK8sApiCoreV1NodeDaemonEndpoints**](io.k8s.api.core.v1.NodeDaemonEndpoints.md) |  | [optional] 
**images** | [**Vec<::models::IoK8sApiCoreV1ContainerImage>**](io.k8s.api.core.v1.ContainerImage.md) | List of container images on this node | [optional] 
**node_info** | [***::models::IoK8sApiCoreV1NodeSystemInfo**](io.k8s.api.core.v1.NodeSystemInfo.md) |  | [optional] 
**phase** | **String** | NodePhase is the recently observed lifecycle phase of the node. More info: https://kubernetes.io/docs/concepts/nodes/node/#phase The field is never populated, and now is deprecated. | [optional] 
**volumes_attached** | [**Vec<::models::IoK8sApiCoreV1AttachedVolume>**](io.k8s.api.core.v1.AttachedVolume.md) | List of volumes that are attached to the node. | [optional] 
**volumes_in_use** | **Vec<String>** | List of attachable volumes in use (mounted) by the node. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


