# IoK8sApiAutoscalingV2beta2HorizontalPodAutoscalerStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**conditions** | [**Vec<::models::IoK8sApiAutoscalingV2beta2HorizontalPodAutoscalerCondition>**](io.k8s.api.autoscaling.v2beta2.HorizontalPodAutoscalerCondition.md) | conditions is the set of conditions required for this autoscaler to scale its target, and indicates whether or not those conditions are met. | 
**current_metrics** | [**Vec<::models::IoK8sApiAutoscalingV2beta2MetricStatus>**](io.k8s.api.autoscaling.v2beta2.MetricStatus.md) | currentMetrics is the last read state of the metrics used by this autoscaler. | [optional] 
**current_replicas** | **i32** | currentReplicas is current number of replicas of pods managed by this autoscaler, as last seen by the autoscaler. | 
**desired_replicas** | **i32** | desiredReplicas is the desired number of replicas of pods managed by this autoscaler, as last calculated by the autoscaler. | 
**last_scale_time** | **String** | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | [optional] 
**observed_generation** | **i64** | observedGeneration is the most recent generation observed by this autoscaler. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


