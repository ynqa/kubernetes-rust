# IoK8sApiCoreV1NodeSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**config_source** | [***::models::IoK8sApiCoreV1NodeConfigSource**](io.k8s.api.core.v1.NodeConfigSource.md) |  | [optional] 
**external_id** | **String** | Deprecated. Not all kubelets will set this field. Remove field after 1.13. see: https://issues.k8s.io/61966 | [optional] 
**pod_cidr** | **String** | PodCIDR represents the pod IP range assigned to the node. | [optional] 
**provider_id** | **String** | ID of the node assigned by the cloud provider in the format: <ProviderName>://<ProviderSpecificNodeID> | [optional] 
**taints** | [**Vec<::models::IoK8sApiCoreV1Taint>**](io.k8s.api.core.v1.Taint.md) | If specified, the node's taints. | [optional] 
**unschedulable** | **bool** | Unschedulable controls node schedulability of new pods. By default, node is schedulable. More info: https://kubernetes.io/docs/concepts/nodes/node/#manual-node-administration | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


