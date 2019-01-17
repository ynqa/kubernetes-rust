# IoK8sApiAutoscalingV1ScaleStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**replicas** | **i32** | actual number of observed instances of the scaled object. | 
**selector** | **String** | label query over pods that should match the replicas count. This is same as the label selector but in the string format to avoid introspection by clients. The string will be in the same format as the query-param syntax. More info about label selectors: http://kubernetes.io/docs/user-guide/labels#label-selectors | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


