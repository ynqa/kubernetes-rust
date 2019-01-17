# IoK8sApiAutoscalingV2beta2MetricSpec

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external** | [***::models::IoK8sApiAutoscalingV2beta2ExternalMetricSource**](io.k8s.api.autoscaling.v2beta2.ExternalMetricSource.md) |  | [optional] 
**object** | [***::models::IoK8sApiAutoscalingV2beta2ObjectMetricSource**](io.k8s.api.autoscaling.v2beta2.ObjectMetricSource.md) |  | [optional] 
**pods** | [***::models::IoK8sApiAutoscalingV2beta2PodsMetricSource**](io.k8s.api.autoscaling.v2beta2.PodsMetricSource.md) |  | [optional] 
**resource** | [***::models::IoK8sApiAutoscalingV2beta2ResourceMetricSource**](io.k8s.api.autoscaling.v2beta2.ResourceMetricSource.md) |  | [optional] 
**_type** | **String** | type is the type of metric source.  It should be one of \"Object\", \"Pods\" or \"Resource\", each mapping to a matching field in the object. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


