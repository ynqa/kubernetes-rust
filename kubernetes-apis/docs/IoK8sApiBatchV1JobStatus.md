# IoK8sApiBatchV1JobStatus

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**active** | **i32** | The number of actively running pods. | [optional] 
**completion_time** | **String** | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | [optional] 
**conditions** | [**Vec<::models::IoK8sApiBatchV1JobCondition>**](io.k8s.api.batch.v1.JobCondition.md) | The latest available observations of an object's current state. More info: https://kubernetes.io/docs/concepts/workloads/controllers/jobs-run-to-completion/ | [optional] 
**failed** | **i32** | The number of pods which reached phase Failed. | [optional] 
**start_time** | **String** | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | [optional] 
**succeeded** | **i32** | The number of pods which reached phase Succeeded. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


