# IoK8sApiBatchV1JobCondition

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_probe_time** | **String** | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | [optional] 
**last_transition_time** | **String** | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | [optional] 
**message** | **String** | Human readable message indicating details about last transition. | [optional] 
**reason** | **String** | (brief) reason for the condition's last transition. | [optional] 
**status** | **String** | Status of the condition, one of True, False, Unknown. | 
**_type** | **String** | Type of job condition, Complete or Failed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


