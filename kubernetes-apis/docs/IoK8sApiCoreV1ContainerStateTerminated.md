# IoK8sApiCoreV1ContainerStateTerminated

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**container_id** | **String** | Container's ID in the format 'docker://<container_id>' | [optional] 
**exit_code** | **i32** | Exit status from the last termination of the container | 
**finished_at** | **String** | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | [optional] 
**message** | **String** | Message regarding the last termination of the container | [optional] 
**reason** | **String** | (brief) reason from the last termination of the container | [optional] 
**signal** | **i32** | Signal from the last termination of the container | [optional] 
**started_at** | **String** | Time is a wrapper around time.Time which supports correct marshaling to YAML and JSON.  Wrappers are provided for many of the factory methods that the time package offers. | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


