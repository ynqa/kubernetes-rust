# IoK8sApiAutoscalingV2beta1MetricStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external** | [***::models::IoK8sApiAutoscalingV2beta1ExternalMetricStatus**](io.k8s.api.autoscaling.v2beta1.ExternalMetricStatus.md) |  | [optional] 
**object** | [***::models::IoK8sApiAutoscalingV2beta1ObjectMetricStatus**](io.k8s.api.autoscaling.v2beta1.ObjectMetricStatus.md) |  | [optional] 
**pods** | [***::models::IoK8sApiAutoscalingV2beta1PodsMetricStatus**](io.k8s.api.autoscaling.v2beta1.PodsMetricStatus.md) |  | [optional] 
**resource** | [***::models::IoK8sApiAutoscalingV2beta1ResourceMetricStatus**](io.k8s.api.autoscaling.v2beta1.ResourceMetricStatus.md) |  | [optional] 
**_type** | **String** | type is the type of metric source.  It will be one of \"Object\", \"Pods\" or \"Resource\", each corresponds to a matching field in the object. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


